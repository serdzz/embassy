//! General purpose input/output.
//!
//! MSP430 groups its 8-bit ports into 16-bit pairs (P1/P2 are the two halves of PA, and so on), so
//! every register of an odd-numbered port sits one byte below the same register of the port above
//! it. That is all this module hides: a pin knows its port and bit, and the register addresses
//! follow from those.
//!
//! Only P1 through P4 can raise interrupts, so [`Input::wait_for_any_edge`] and friends are
//! available on those ports only; asking for them on P5 or P6 panics.

use core::convert::Infallible;
use core::future::Future;
use core::pin::Pin as FuturePin;
use core::task::{Context, Poll};

use embassy_hal_internal::{Peri, PeripheralType, impl_peripheral};
use embassy_sync::waitqueue::AtomicWaker;

/// Number of pins that can raise an interrupt: P1 through P4, 8 bits each.
const IRQ_PINS: usize = 32;

#[allow(clippy::declare_interior_mutable_const)]
const NEW_AW: AtomicWaker = AtomicWaker::new();
static PORT_WAKERS: [AtomicWaker; IRQ_PINS] = [NEW_AW; IRQ_PINS];

/// Digital input or output level.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Level {
    /// Low, 0V.
    Low,
    /// High, VCC.
    High,
}

impl From<bool> for Level {
    fn from(val: bool) -> Self {
        if val { Level::High } else { Level::Low }
    }
}

impl From<Level> for bool {
    fn from(level: Level) -> bool {
        level == Level::High
    }
}

/// Pull setting for an input.
///
/// MSP430 has a single resistor per pin whose direction is chosen by the output latch, so a pin
/// cannot be pulled up and down at once.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    /// No pull, the pin floats.
    None,
    /// Pull up to VCC.
    Up,
    /// Pull down to ground.
    Down,
}

// Register offsets within a port pair. The odd-numbered port of a pair uses these directly; the
// even-numbered one adds a byte.
const IN: u16 = 0x00;
const OUT: u16 = 0x02;
const DIR: u16 = 0x04;
const REN: u16 = 0x06;
const SEL0: u16 = 0x0a;
const SEL1: u16 = 0x0c;
const IES: u16 = 0x18;
const IE: u16 = 0x1a;
const IFG: u16 = 0x1c;

const fn port_base(port: u8) -> u16 {
    // PA = P1/P2 at 0x0200, PB = P3/P4 at 0x0220, PC = P5/P6 at 0x0240.
    //
    // Written as shifts rather than a multiply on purpose: MSP430 has no multiply instruction, so
    // `*` would become a call to `__mspabi_mpyi` even here.
    0x0200 + ((port as u16 >> 1) << 5)
}

const fn reg(port: u8, offset: u16) -> *mut u8 {
    (port_base(port) + offset + (port as u16 & 1)) as *mut u8
}

/// Address of the port's interrupt vector register, which reports and clears the highest-priority
/// pending flag in one read.
const fn iv_reg(port: u8) -> *mut u16 {
    (port_base(port) + if port & 1 == 0 { 0x0e } else { 0x1e }) as *mut u16
}

/// Read-modify-write a port register.
///
/// Ports are shared between pins, and the interrupt registers are also touched from the port ISR,
/// so the read and the write have to be one indivisible step.
#[inline]
fn modify(port: u8, offset: u16, f: impl FnOnce(u8) -> u8) {
    critical_section::with(|_| {
        let r = reg(port, offset);
        // SAFETY: `r` is a valid port register for an existing port, and the critical section
        // keeps the read-modify-write from racing another pin or the port ISR.
        unsafe { r.write_volatile(f(r.read_volatile())) }
    })
}

#[inline]
fn read(port: u8, offset: u16) -> u8 {
    // SAFETY: a plain volatile read of an existing port register.
    unsafe { reg(port, offset).read_volatile() }
}

/// Hand the pin to its first alternate function (`SEL1:SEL0 = 01`), which is where the eUSCI and
/// timer signals live on this family.
pub(crate) fn set_alternate1(pin: &AnyPin) {
    let port = pin.pin_port >> 3;
    let bit = 1u8 << (pin.pin_port & 7);
    modify(port, SEL1, |v| v & !bit);
    modify(port, SEL0, |v| v | bit);
}

/// Hand the pin to its second alternate function (`SEL1:SEL0 = 10`).
pub(crate) fn set_alternate2(pin: &AnyPin) {
    let port = pin.pin_port >> 3;
    let bit = 1u8 << (pin.pin_port & 7);
    modify(port, SEL0, |v| v & !bit);
    modify(port, SEL1, |v| v | bit);
}

/// Hand the pin to its third alternate function (`SEL1:SEL0 = 11`), which on this family is the
/// analog one: the digital input buffer is disconnected, so a mid-rail analog voltage cannot make
/// it oscillate.
pub(crate) fn set_analog(pin: &impl SealedPin) {
    let pin_port = pin.pin_port();
    let port = pin_port >> 3;
    let bit = 1u8 << (pin_port & 7);
    // Take it out of any output mode first, otherwise the pin fights the source being measured.
    modify(port, DIR, |v| v & !bit);
    modify(port, REN, |v| v & !bit);
    modify(port, SEL0, |v| v | bit);
    modify(port, SEL1, |v| v | bit);
}

/// Take the pin back from whatever peripheral had it.
pub(crate) fn set_gpio_function(pin: &AnyPin) {
    let port = pin.pin_port >> 3;
    let bit = 1u8 << (pin.pin_port & 7);
    modify(port, SEL0, |v| v & !bit);
    modify(port, SEL1, |v| v & !bit);
}

/// A GPIO pin with its mode chosen at runtime.
pub struct Flex<'d> {
    pin: Peri<'d, AnyPin>,
}

impl<'d> Flex<'d> {
    /// Wrap the pin, leaving its current configuration alone.
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>) -> Self {
        let this = Self { pin: pin.into() };
        // Take the pin away from whatever peripheral had it multiplexed.
        modify(this.port(), SEL0, |v| v & !this.bit());
        modify(this.port(), SEL1, |v| v & !this.bit());
        this
    }

    #[inline]
    fn port(&self) -> u8 {
        self.pin.pin_port() / 8
    }

    #[inline]
    fn bit(&self) -> u8 {
        1 << (self.pin.pin_port() % 8)
    }

    /// Put the pin into input mode with the given pull.
    #[inline]
    pub fn set_as_input(&mut self, pull: Pull) {
        let bit = self.bit();
        let port = self.port();
        match pull {
            Pull::None => modify(port, REN, |v| v & !bit),
            // With the pin an input, the output latch picks which way the resistor pulls.
            Pull::Up => {
                modify(port, OUT, |v| v | bit);
                modify(port, REN, |v| v | bit);
            }
            Pull::Down => {
                modify(port, OUT, |v| v & !bit);
                modify(port, REN, |v| v | bit);
            }
        }
        modify(port, DIR, |v| v & !bit);
    }

    /// Put the pin into push-pull output mode.
    ///
    /// The level is whatever the output latch already held; set it first if that matters.
    #[inline]
    pub fn set_as_output(&mut self) {
        let bit = self.bit();
        modify(self.port(), REN, |v| v & !bit);
        modify(self.port(), DIR, |v| v | bit);
    }

    /// Is the input level high?
    #[inline]
    pub fn is_high(&self) -> bool {
        read(self.port(), IN) & self.bit() != 0
    }

    /// Is the input level low?
    #[inline]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// Current input level.
    #[inline]
    pub fn get_level(&self) -> Level {
        self.is_high().into()
    }

    /// Drive the pin high.
    #[inline]
    pub fn set_high(&mut self) {
        let bit = self.bit();
        modify(self.port(), OUT, |v| v | bit);
    }

    /// Drive the pin low.
    #[inline]
    pub fn set_low(&mut self) {
        let bit = self.bit();
        modify(self.port(), OUT, |v| v & !bit);
    }

    /// Drive the pin to `level`.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }

    /// Flip the output level.
    #[inline]
    pub fn toggle(&mut self) {
        let bit = self.bit();
        modify(self.port(), OUT, |v| v ^ bit);
    }

    /// Is the output latch driving high?
    ///
    /// This reads the latch, not the pin, so it answers what the pin is being told to do rather
    /// than what it actually is.
    #[inline]
    pub fn is_set_high(&self) -> bool {
        read(self.port(), OUT) & self.bit() != 0
    }

    /// Is the output latch driving low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    /// Wait for the pin to go high. Returns immediately if it already is.
    pub async fn wait_for_high(&mut self) {
        if self.is_high() {
            return;
        }
        self.wait_for_edge(true).await;
    }

    /// Wait for the pin to go low. Returns immediately if it already is.
    pub async fn wait_for_low(&mut self) {
        if self.is_low() {
            return;
        }
        self.wait_for_edge(false).await;
    }

    /// Wait for a low-to-high transition.
    pub async fn wait_for_rising_edge(&mut self) {
        self.wait_for_edge(true).await;
    }

    /// Wait for a high-to-low transition.
    pub async fn wait_for_falling_edge(&mut self) {
        self.wait_for_edge(false).await;
    }

    /// Wait for the pin to change level in either direction.
    ///
    /// The edge is chosen from the level at the time of the call, so this waits for the pin to
    /// leave the state it is in now.
    pub async fn wait_for_any_edge(&mut self) {
        let rising = self.is_low();
        self.wait_for_edge(rising).await;
    }

    fn wait_for_edge(&mut self, rising: bool) -> PortInputFuture<'_, 'd> {
        let port = self.port();
        assert!(
            port < 4,
            "only P1..P4 can raise interrupts on this chip, P{} cannot",
            port + 1
        );
        let bit = self.bit();

        critical_section::with(|_| {
            // IES picks the edge: 0 is low-to-high, 1 is high-to-low.
            modify(port, IES, |v| if rising { v & !bit } else { v | bit });
            // Selecting an edge can itself set the flag, so clear it after choosing.
            modify(port, IFG, |v| v & !bit);
            modify(port, IE, |v| v | bit);
        });

        PortInputFuture { pin: self }
    }
}

impl<'d> Drop for Flex<'d> {
    fn drop(&mut self) {
        let bit = self.bit();
        let port = self.port();
        if port < 4 {
            modify(port, IE, |v| v & !bit);
            modify(port, IFG, |v| v & !bit);
        }
        modify(port, DIR, |v| v & !bit);
        modify(port, REN, |v| v & !bit);
    }
}

/// Resolves once the pin's interrupt has fired.
///
/// The port ISR masks the pin, so "has it fired" is simply "is the pin still enabled".
struct PortInputFuture<'a, 'd> {
    pin: &'a mut Flex<'d>,
}

impl<'a, 'd> Drop for PortInputFuture<'a, 'd> {
    fn drop(&mut self) {
        let bit = self.pin.bit();
        let port = self.pin.port();
        modify(port, IE, |v| v & !bit);
        modify(port, IFG, |v| v & !bit);
    }
}

impl<'a, 'd> Future for PortInputFuture<'a, 'd> {
    type Output = ();

    fn poll(self: FuturePin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let pin_port = self.pin.pin.pin_port();
        PORT_WAKERS[pin_port as usize].register(cx.waker());

        if read(pin_port / 8, IE) & (1 << (pin_port % 8)) == 0 {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

/// A GPIO input pin.
pub struct Input<'d> {
    pin: Flex<'d>,
}

impl<'d> Input<'d> {
    /// Configure the pin as an input with the given pull.
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, pull: Pull) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_as_input(pull);
        Self { pin }
    }

    /// Is the input level high?
    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    /// Is the input level low?
    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    /// Current input level.
    #[inline]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }

    /// Wait for the pin to go high. Returns immediately if it already is.
    pub async fn wait_for_high(&mut self) {
        self.pin.wait_for_high().await
    }

    /// Wait for the pin to go low. Returns immediately if it already is.
    pub async fn wait_for_low(&mut self) {
        self.pin.wait_for_low().await
    }

    /// Wait for a low-to-high transition.
    pub async fn wait_for_rising_edge(&mut self) {
        self.pin.wait_for_rising_edge().await
    }

    /// Wait for a high-to-low transition.
    pub async fn wait_for_falling_edge(&mut self) {
        self.pin.wait_for_falling_edge().await
    }

    /// Wait for the pin to change level in either direction.
    pub async fn wait_for_any_edge(&mut self) {
        self.pin.wait_for_any_edge().await
    }
}

/// A GPIO output pin.
pub struct Output<'d> {
    pin: Flex<'d>,
}

impl<'d> Output<'d> {
    /// Configure the pin as a push-pull output starting at `initial_level`.
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, initial_level: Level) -> Self {
        let mut pin = Flex::new(pin);
        // Set the latch first, so the pin never briefly drives the wrong level.
        pin.set_level(initial_level);
        pin.set_as_output();
        Self { pin }
    }

    /// Drive the pin high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high()
    }

    /// Drive the pin low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low()
    }

    /// Drive the pin to `level`.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level)
    }

    /// Flip the output level.
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle()
    }

    /// Is the output latch driving high?
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high()
    }

    /// Is the output latch driving low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.pin.is_set_low()
    }
}

/// A pin, with its identity erased.
pub struct AnyPin {
    pin_port: u8,
}

impl AnyPin {
    /// Build an `AnyPin` from a port index (0 for P1) and a bit number.
    ///
    /// # Safety
    ///
    /// You must make sure the pin is not in use anywhere else.
    #[inline]
    pub unsafe fn steal(port: u8, pin: u8) -> Peri<'static, Self> {
        unsafe {
            Peri::new_unchecked(Self {
                pin_port: (port << 3) | pin,
            })
        }
    }
}

impl_peripheral!(AnyPin);
impl SealedPin for AnyPin {
    #[inline]
    fn pin_port(&self) -> u8 {
        self.pin_port
    }
}
impl Pin for AnyPin {}

pub(crate) trait SealedPin {
    /// Port index times 8, plus the bit number.
    fn pin_port(&self) -> u8;
}

/// A GPIO pin.
#[allow(private_bounds)]
pub trait Pin: PeripheralType + Into<AnyPin> + SealedPin + Sized + 'static {
    /// Bit number within the port, 0 to 7.
    #[inline]
    fn pin(&self) -> u8 {
        self.pin_port() % 8
    }

    /// Port index: 0 for P1, 1 for P2, and so on.
    #[inline]
    fn port(&self) -> u8 {
        self.pin_port() / 8
    }

    /// Erase the pin's type.
    #[inline]
    fn degrade(self) -> AnyPin {
        AnyPin {
            pin_port: self.pin_port(),
        }
    }
}

macro_rules! impl_pin {
    ($name:ident, $port:expr, $pin:expr) => {
        impl crate::gpio::SealedPin for crate::peripherals::$name {
            #[inline]
            fn pin_port(&self) -> u8 {
                $port * 8 + $pin
            }
        }
        impl crate::gpio::Pin for crate::peripherals::$name {}
        impl From<crate::peripherals::$name> for crate::gpio::AnyPin {
            fn from(val: crate::peripherals::$name) -> Self {
                crate::gpio::Pin::degrade(val)
            }
        }
    };
}
pub(crate) use impl_pin;

/// Handle a port interrupt.
///
/// `PxIV` reports the highest-priority pending pin and clears its flag in the same read, so this
/// drains one pin per call and the hardware re-raises the interrupt if more are pending.
fn on_port_irq(port: u8) {
    // SAFETY: `port` is one of the interrupt-capable ports, so this is a real IV register.
    let iv = unsafe { iv_reg(port).read_volatile() };
    if iv == 0 {
        return;
    }
    let pin = (iv / 2 - 1) as u8;
    let bit = 1u8 << pin;

    // Mask the pin: that is how the future learns its edge arrived, and it stops the interrupt
    // from firing again before the task has had a chance to run.
    modify(port, IE, |v| v & !bit);
    PORT_WAKERS[((port << 3) | pin) as usize].wake();
}

embassy_executor::msp430_interrupt! {
    /// Port 1 edge.
    unsafe fn PORT1() {
        on_port_irq(0);
    }

    /// Port 2 edge.
    unsafe fn PORT2() {
        on_port_irq(1);
    }

    /// Port 3 edge.
    unsafe fn PORT3() {
        on_port_irq(2);
    }

    /// Port 4 edge.
    unsafe fn PORT4() {
        on_port_irq(3);
    }
}

impl<'d> embedded_hal::digital::ErrorType for Flex<'d> {
    type Error = Infallible;
}

impl<'d> embedded_hal::digital::InputPin for Flex<'d> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_low())
    }
}

impl<'d> embedded_hal::digital::OutputPin for Flex<'d> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(self.set_high())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.set_low())
    }
}

impl<'d> embedded_hal::digital::StatefulOutputPin for Flex<'d> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_high())
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_low())
    }
}

impl<'d> embedded_hal::digital::ErrorType for Input<'d> {
    type Error = Infallible;
}

impl<'d> embedded_hal::digital::InputPin for Input<'d> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_low())
    }
}

impl<'d> embedded_hal::digital::ErrorType for Output<'d> {
    type Error = Infallible;
}

impl<'d> embedded_hal::digital::OutputPin for Output<'d> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(self.set_high())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.set_low())
    }
}

impl<'d> embedded_hal::digital::StatefulOutputPin for Output<'d> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_high())
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_low())
    }
}

impl<'d> embedded_hal_async::digital::Wait for Flex<'d> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_high().await)
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_low().await)
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_rising_edge().await)
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_falling_edge().await)
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_any_edge().await)
    }
}

impl<'d> embedded_hal_async::digital::Wait for Input<'d> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_high().await)
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_low().await)
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_rising_edge().await)
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_falling_edge().await)
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        Ok(self.wait_for_any_edge().await)
    }
}
