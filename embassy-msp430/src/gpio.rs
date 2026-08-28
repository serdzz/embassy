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

use crate::chip::{self, PinFunction, PortReg};

/// Number of pins that can raise an interrupt: eight per interrupt-capable port.
const IRQ_PINS: usize = chip::IRQ_PORTS as usize * 8;

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

// The offsets themselves live in `chip`; these names keep the call sites readable.
const IN: PortReg = PortReg::In;
const OUT: PortReg = PortReg::Out;
const DIR: PortReg = PortReg::Dir;
const REN: PortReg = PortReg::Ren;
const IES: PortReg = PortReg::Ies;
const IE: PortReg = PortReg::Ie;
const IFG: PortReg = PortReg::Ifg;

/// Address of one of a port's registers.
///
/// Panics if the device has no such register, which only a driver bug can ask for: the pull
/// resistors are guarded by [`chip::HAS_PULL`] and the interrupt registers by
/// [`chip::IRQ_PORTS`].
#[inline]
fn reg(port: u8, which: PortReg) -> *mut u8 {
    match chip::port_reg(port, which) {
        Some(r) => r,
        None => panic!("this device has no {:?} register for P{}", which, port + 1),
    }
}

/// Read-modify-write a port register.
///
/// Ports are shared between pins, and the interrupt registers are also touched from the port ISR,
/// so the read and the write have to be one indivisible step.
#[inline]
fn modify(port: u8, which: PortReg, f: impl FnOnce(u8) -> u8) {
    critical_section::with(|_| {
        let r = reg(port, which);
        // SAFETY: `r` is a valid port register for an existing port, and the critical section
        // keeps the read-modify-write from racing another pin or the port ISR.
        unsafe { r.write_volatile(f(r.read_volatile())) }
    })
}

#[inline]
fn read(port: u8, which: PortReg) -> u8 {
    // SAFETY: a plain volatile read of an existing port register.
    unsafe { reg(port, which).read_volatile() }
}

/// Set or clear one bit of a register the caller has already located.
///
/// Used by the `chip` modules, whose function-select registers do not fit [`PortReg`].
#[inline]
pub(crate) fn modify_reg(r: *mut u8, bit: u8, set: bool) {
    critical_section::with(|_| {
        // SAFETY: the caller passes a port register address, and the critical section makes the
        // read-modify-write indivisible.
        unsafe {
            let v = r.read_volatile();
            r.write_volatile(if set { v | bit } else { v & !bit });
        }
    })
}

/// Hand the pin to its first alternate function, which is where the serial peripherals and the
/// timer outputs live.
// Only called by the peripheral drivers, which not every device has.
#[allow(dead_code)]
pub(crate) fn set_alternate1(pin: &AnyPin) {
    chip::set_pin_function(pin.pin_port >> 3, 1u8 << (pin.pin_port & 7), PinFunction::Alternate1);
}

/// Hand the pin to its second alternate function.
// Only called by the peripheral drivers, which not every device has.
#[allow(dead_code)]
pub(crate) fn set_alternate2(pin: &AnyPin) {
    chip::set_pin_function(pin.pin_port >> 3, 1u8 << (pin.pin_port & 7), PinFunction::Alternate2);
}

/// Hand the pin to the analog function, which disconnects the digital input buffer so a mid-rail
/// voltage cannot make it oscillate.
// Only called by the peripheral drivers, which not every device has.
#[allow(dead_code)]
pub(crate) fn set_analog(pin: &impl SealedPin) {
    let pin_port = pin.pin_port();
    let port = pin_port >> 3;
    let bit = 1u8 << (pin_port & 7);
    // Take it out of any output mode first, otherwise the pin fights the source being measured.
    modify(port, DIR, |v| v & !bit);
    if chip::HAS_PULL {
        modify(port, REN, |v| v & !bit);
    }
    chip::set_pin_function(port, bit, chip::ANALOG_FUNCTION);
}

/// Take the pin back from whatever peripheral had it.
pub(crate) fn set_gpio_function(pin: &AnyPin) {
    chip::set_pin_function(pin.pin_port >> 3, 1u8 << (pin.pin_port & 7), PinFunction::Gpio);
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
        set_gpio_function(&this.pin);
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
        if !chip::HAS_PULL {
            assert!(
                pull == Pull::None,
                "this device has no pull resistors; the pin needs an external one"
            );
            modify(port, DIR, |v| v & !bit);
            return;
        }

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
        if chip::HAS_PULL {
            modify(self.port(), REN, |v| v & !bit);
        }
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
            port < chip::IRQ_PORTS,
            "only P1..P{} can raise interrupts on this device, P{} cannot",
            chip::IRQ_PORTS,
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
        if port < chip::IRQ_PORTS {
            modify(port, IE, |v| v & !bit);
            modify(port, IFG, |v| v & !bit);
        }
        modify(port, DIR, |v| v & !bit);
        if chip::HAS_PULL {
            modify(port, REN, |v| v & !bit);
        }
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
/// Only one pin is dealt with per call. Where the hardware has a vector register that is inherent:
/// it reports the highest-priority flag and clears it. Where it does not, taking the lowest pending
/// pin keeps the two paths the same shape, and the hardware re-raises the interrupt for the rest.
fn on_port_irq(port: u8) {
    let pin = if chip::HAS_PORT_IV {
        // SAFETY: `port` is interrupt-capable, so this is a real vector register. Reading it is
        // what clears the flag it reports.
        let iv = unsafe { chip::port_iv(port).read_volatile() };
        if iv == 0 {
            return;
        }
        (iv / 2 - 1) as u8
    } else {
        let pending = read(port, IFG) & read(port, IE);
        if pending == 0 {
            return;
        }
        pending.trailing_zeros() as u8
    };
    let bit = 1u8 << pin;

    // Mask the pin: that is how the future learns its edge arrived, and it stops the interrupt from
    // firing again before the task has had a chance to run.
    modify(port, IE, |v| v & !bit);
    if !chip::HAS_PORT_IV {
        // Nothing cleared the flag on the way in.
        modify(port, IFG, |v| v & !bit);
    }
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
}

#[cfg(feature = "msp430fr2355")]
embassy_executor::msp430_interrupt! {
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
