//! Universal asynchronous receiver/transmitter (eUSCI_A in UART mode).
//!
//! On the MSP-EXP430FR2355 LaunchPad, `EUSCI_A1` (P4.3/P4.2) is wired to the eZ-FET backchannel,
//! so it is the one that shows up as a serial port on the host.
//!
//! # Baud rate
//!
//! The bit clock is derived from SMCLK or ACLK. Out of reset SMCLK is only about 1 MHz, which is
//! not much of a margin at 115200 baud; raise [`crate::clock::DcoFreq`] or pick a slower rate if
//! the error matters. [`Config::baudrate`] is honoured as closely as the oversampling and
//! modulation hardware allows, and [`Uart::new`] reports [`ConfigError::UnachievableBaudrate`] when
//! it cannot get close at all.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};

use crate::clock::PeripheralClock;
use crate::eusci::{self, Info};
use crate::gpio::{self, AnyPin, Pin};
use crate::peripherals;

// Control word 0.
const UCSWRST: u16 = 0x0001;
const UCSSEL_ACLK: u16 = 0x0040;
const UCSSEL_SMCLK: u16 = 0x0080;
const UCSPB: u16 = 0x0800;
const UC7BIT: u16 = 0x1000;
const UCPAR_EVEN: u16 = 0x4000;
const UCPEN: u16 = 0x8000;

// Modulation control word.
const UCOS16: u16 = 0x0001;

// Status word.
const UCBUSY: u16 = 0x0001;
const UCBRK: u16 = 0x0008;
const UCPE: u16 = 0x0010;
const UCOE: u16 = 0x0020;
const UCFE: u16 = 0x0040;

// Interrupt enable and flags.
const UCRXIE: u16 = 0x0001;
const UCTXIE: u16 = 0x0002;
const UCTXCPTIE: u16 = 0x0008;
const UCRXIFG: u16 = 0x0001;
const UCTXIFG: u16 = 0x0002;
const UCTXCPTIFG: u16 = 0x0008;

// Register offsets from the peripheral base.
const CTLW0: u16 = 0x00;
const BRW: u16 = 0x06;
const MCTLW: u16 = 0x08;
const STATW: u16 = 0x0a;
const RXBUF: u16 = 0x0c;
const TXBUF: u16 = 0x0e;

/// Number of bits per character.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataBits {
    /// 7 data bits.
    Seven,
    /// 8 data bits.
    #[default]
    Eight,
}

/// Number of stop bits.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopBits {
    /// One stop bit.
    #[default]
    One,
    /// Two stop bits.
    Two,
}

/// Parity.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    /// No parity bit.
    #[default]
    None,
    /// Even parity.
    Even,
    /// Odd parity.
    Odd,
}

/// UART configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// Baud rate.
    pub baudrate: u32,
    /// Number of data bits.
    pub data_bits: DataBits,
    /// Number of stop bits.
    pub stop_bits: StopBits,
    /// Parity.
    pub parity: Parity,
    /// Clock the bit rate generator runs from.
    pub clock_source: PeripheralClock,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baudrate: 115_200,
            data_bits: DataBits::default(),
            stop_bits: StopBits::default(),
            parity: Parity::default(),
            clock_source: PeripheralClock::default(),
        }
    }
}

/// Reasons a [`Config`] cannot be applied.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// The bit rate generator cannot reach this baud rate from the selected clock.
    UnachievableBaudrate,
    /// [`crate::init`] has not run, so the clock frequencies are unknown.
    ClocksNotInitialized,
}

/// UART errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// A start or stop bit arrived at the wrong time.
    Framing,
    /// The parity bit did not match.
    Parity,
    /// A character arrived before the previous one had been read.
    Overrun,
    /// A break condition was received.
    Break,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Error::Framing => "framing error",
            Error::Parity => "parity error",
            Error::Overrun => "receive overrun",
            Error::Break => "break received",
        };
        f.write_str(s)
    }
}

impl core::error::Error for Error {}

impl embedded_io_async::Error for Error {
    fn kind(&self) -> embedded_io_async::ErrorKind {
        embedded_io_async::ErrorKind::Other
    }
}

/// Bit rate generator settings.
struct BaudConfig {
    brw: u16,
    mctlw: u16,
}

/// Work out `UCBRx`, `UCBRFx`, `UCBRSx` and `UCOS16` for `baudrate` from a `clock` Hz bit clock.
///
/// Above 16 clocks per bit the hardware can oversample, which both tolerates more clock error and
/// lets the fractional part be dialled in with `UCBRFx`. Below that it counts whole clocks and only
/// `UCBRSx` is left to spread the error across a character.
fn calc_baud(clock: u32, baudrate: u32) -> Result<BaudConfig, ConfigError> {
    if baudrate == 0 || clock == 0 {
        return Err(ConfigError::UnachievableBaudrate);
    }

    let n = clock / baudrate;
    if n == 0 || n > 0xFFFF {
        return Err(ConfigError::UnachievableBaudrate);
    }

    let brs = (lookup_brs(clock, baudrate) as u16) << 8;

    if n >= 16 {
        let div = baudrate * 16;
        let brw = (clock / div) as u16;
        // The same as `n % 16`, but computed before the division throws the precision away.
        let brf = ((clock % div) / baudrate) as u16;
        Ok(BaudConfig {
            brw,
            mctlw: brs | (brf << 4) | UCOS16,
        })
    } else {
        Ok(BaudConfig {
            brw: n as u16,
            mctlw: brs,
        })
    }
}

/// `UCBRSx` for the fractional part of `clock / baudrate`.
///
/// Straight out of Table 22-4 of the MSP430FR4xx and MSP430FR2xx family user's guide: the value is
/// a bit pattern of which bit times get an extra clock, not a number, so it has to be looked up.
fn lookup_brs(clock: u32, baudrate: u32) -> u8 {
    let modulo = clock % baudrate;
    // Fraction of a bit time, in ten-thousandths. Scaled in two steps when the numerator would
    // otherwise overflow a u32.
    let frac = if modulo < u32::MAX / 10_000 {
        (modulo * 10_000) / baudrate
    } else {
        ((modulo * 500) / baudrate) * 20
    } as u16;

    const TABLE: [(u16, u8); 36] = [
        (529, 0x00),
        (715, 0x01),
        (835, 0x02),
        (1001, 0x04),
        (1252, 0x08),
        (1430, 0x10),
        (1670, 0x20),
        (2147, 0x11),
        (2224, 0x21),
        (2503, 0x22),
        (3000, 0x44),
        (3335, 0x25),
        (3575, 0x49),
        (3753, 0x4A),
        (4003, 0x52),
        (4286, 0x92),
        (4378, 0x53),
        (5002, 0x55),
        (5715, 0xAA),
        (6003, 0x6B),
        (6254, 0xAD),
        (6432, 0xB5),
        (6667, 0xB6),
        (7001, 0xD6),
        (7147, 0xB7),
        (7503, 0xBB),
        (7861, 0xDD),
        (8004, 0xED),
        (8333, 0xEE),
        (8464, 0xBF),
        (8572, 0xDF),
        (8751, 0xEF),
        (9004, 0xF7),
        (9170, 0xFB),
        (9288, 0xFD),
        (u16::MAX, 0xFE),
    ];

    let mut i = 0;
    while i < TABLE.len() {
        if frac < TABLE[i].0 {
            return TABLE[i].1;
        }
        i += 1;
    }
    0xFE
}

/// Turn the receive status bits into an error.
///
/// Must be read before `RXBUF`: reading the character clears the status along with the flag.
fn check_rx_error(info: &Info) -> Result<(), Error> {
    let statw = info.read(STATW);
    if statw & UCFE != 0 {
        Err(Error::Framing)
    } else if statw & UCPE != 0 {
        Err(Error::Parity)
    } else if statw & UCOE != 0 {
        Err(Error::Overrun)
    } else if statw & UCBRK != 0 {
        Err(Error::Break)
    } else {
        Ok(())
    }
}

trait SealedInstance {
    fn info() -> &'static Info;
}

/// An eUSCI_A instance usable as a UART.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static {}

macro_rules! impl_instance {
    ($peri:ident, $info:ident) => {
        impl SealedInstance for peripherals::$peri {
            fn info() -> &'static Info {
                &eusci::$info
            }
        }
        impl Instance for peripherals::$peri {}
    };
}

impl_instance!(EUSCI_A0, INFO_A0);
impl_instance!(EUSCI_A1, INFO_A1);

/// A pin that can be an instance's transmit line.
#[allow(private_bounds)]
pub trait TxPin<T: Instance>: Pin {}
/// A pin that can be an instance's receive line.
#[allow(private_bounds)]
pub trait RxPin<T: Instance>: Pin {}

impl TxPin<peripherals::EUSCI_A0> for peripherals::P1_7 {}
impl RxPin<peripherals::EUSCI_A0> for peripherals::P1_6 {}
impl TxPin<peripherals::EUSCI_A1> for peripherals::P4_3 {}
impl RxPin<peripherals::EUSCI_A1> for peripherals::P4_2 {}

/// The transmit half of a UART.
pub struct UartTx<'d> {
    info: &'static Info,
    pin: Peri<'d, AnyPin>,
}

/// The receive half of a UART.
pub struct UartRx<'d> {
    info: &'static Info,
    pin: Peri<'d, AnyPin>,
}

/// A bidirectional UART.
pub struct Uart<'d> {
    tx: UartTx<'d>,
    rx: UartRx<'d>,
}

impl<'d> Uart<'d> {
    /// Configure `instance` as a UART on `rx` and `tx`.
    pub fn new<T: Instance>(
        _instance: Peri<'d, T>,
        rx: Peri<'d, impl RxPin<T>>,
        tx: Peri<'d, impl TxPin<T>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let info = T::info();
        let rx: Peri<'d, AnyPin> = rx.into();
        let tx: Peri<'d, AnyPin> = tx.into();

        configure(info, config)?;

        // Hand the pins to the eUSCI only once it is configured, so nothing is driven meanwhile.
        gpio::set_alternate1(&rx);
        gpio::set_alternate1(&tx);

        Ok(Self {
            tx: UartTx { info, pin: tx },
            rx: UartRx { info, pin: rx },
        })
    }

    /// Split into halves that can be moved into separate tasks.
    pub fn split(self) -> (UartTx<'d>, UartRx<'d>) {
        (self.tx, self.rx)
    }

    /// Send every byte of `buf`, waiting for the hardware between characters.
    pub async fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.tx.write(buf).await
    }

    /// Send every byte of `buf`, spinning between characters.
    pub fn blocking_write(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.tx.blocking_write(buf)
    }

    /// Wait until the last character has left the shift register.
    pub async fn flush(&mut self) -> Result<(), Error> {
        self.tx.flush().await
    }

    /// Fill `buf`, waiting for the hardware between characters.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        self.rx.read(buf).await
    }

    /// Fill `buf`, spinning between characters.
    pub fn blocking_read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        self.rx.blocking_read(buf)
    }
}

/// Apply `config` to `info`.
///
/// The eUSCI has to be held in reset for this: most of its control bits are only sampled while
/// `UCSWRST` is set.
fn configure(info: &'static Info, config: Config) -> Result<(), ConfigError> {
    let clocks = crate::clocks().ok_or(ConfigError::ClocksNotInitialized)?;
    let (source_bits, source_hz) = match config.clock_source {
        PeripheralClock::Smclk => (UCSSEL_SMCLK, clocks.smclk),
        PeripheralClock::Aclk => (UCSSEL_ACLK, clocks.aclk),
    };
    let baud = calc_baud(source_hz, config.baudrate)?;

    let mut ctlw0 = UCSWRST | source_bits;
    if config.data_bits == DataBits::Seven {
        ctlw0 |= UC7BIT;
    }
    if config.stop_bits == StopBits::Two {
        ctlw0 |= UCSPB;
    }
    match config.parity {
        Parity::None => {}
        Parity::Even => ctlw0 |= UCPEN | UCPAR_EVEN,
        Parity::Odd => ctlw0 |= UCPEN,
    }

    // Enter reset first: the bit rate registers are only latched while UCSWRST is set.
    info.write(CTLW0, UCSWRST);
    info.write(CTLW0, ctlw0);
    info.write(BRW, baud.brw);
    info.write(MCTLW, baud.mctlw);
    info.write(info.ie_off, 0);
    info.write(info.ifg_off, 0);
    // Release reset. UCTXIFG comes up set, meaning the transmit buffer is empty.
    info.write(CTLW0, ctlw0 & !UCSWRST);

    Ok(())
}

impl<'d> UartTx<'d> {
    /// Send every byte of `buf`, waiting for the hardware between characters.
    pub async fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        for &byte in buf {
            let info = self.info;
            poll_fn(|cx| {
                critical_section::with(|_| {
                    if info.ifg() & UCTXIFG != 0 {
                        info.write(TXBUF, byte as u16);
                        Poll::Ready(())
                    } else {
                        // Register before unmasking: the handler cannot run until this critical
                        // section ends, so the wakeup cannot be missed.
                        info.waker(1).register(cx.waker());
                        info.enable_irq(UCTXIE);
                        Poll::Pending
                    }
                })
            })
            .await;
        }
        Ok(())
    }

    /// Send every byte of `buf`, spinning between characters.
    pub fn blocking_write(&mut self, buf: &[u8]) -> Result<(), Error> {
        for &byte in buf {
            while self.info.ifg() & UCTXIFG == 0 {}
            self.info.write(TXBUF, byte as u16);
        }
        Ok(())
    }

    /// Wait until the last character has left the shift register.
    pub async fn flush(&mut self) -> Result<(), Error> {
        let info = self.info;
        poll_fn(|cx| {
            critical_section::with(|_| {
                if info.read(STATW) & UCBUSY == 0 {
                    Poll::Ready(())
                } else {
                    info.waker(1).register(cx.waker());
                    info.clear_flags(UCTXCPTIFG);
                    info.enable_irq(UCTXCPTIE);
                    Poll::Pending
                }
            })
        })
        .await;
        Ok(())
    }

    /// Wait until the last character has left the shift register, spinning.
    pub fn blocking_flush(&mut self) -> Result<(), Error> {
        while self.info.read(STATW) & UCBUSY != 0 {}
        Ok(())
    }
}

impl<'d> UartRx<'d> {
    /// Fill `buf`, waiting for the hardware between characters.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        for slot in buf.iter_mut() {
            let info = self.info;
            let byte = poll_fn(|cx| {
                critical_section::with(|_| {
                    if info.ifg() & UCRXIFG != 0 {
                        // Check the status before reading the buffer: reading it clears both.
                        let res = check_rx_error(info);
                        let byte = info.read(RXBUF) as u8;
                        Poll::Ready(res.map(|()| byte))
                    } else {
                        info.waker(0).register(cx.waker());
                        info.enable_irq(UCRXIE);
                        Poll::Pending
                    }
                })
            })
            .await?;
            *slot = byte;
        }
        Ok(())
    }

    /// Fill `buf`, spinning between characters.
    pub fn blocking_read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        for slot in buf.iter_mut() {
            while self.info.ifg() & UCRXIFG == 0 {}
            check_rx_error(self.info)?;
            *slot = self.info.read(RXBUF) as u8;
        }
        Ok(())
    }
}

impl<'d> Drop for UartTx<'d> {
    fn drop(&mut self) {
        self.info.disable_irq(UCTXIE | UCTXCPTIE);
        gpio::set_gpio_function(&self.pin);
    }
}

impl<'d> Drop for UartRx<'d> {
    fn drop(&mut self) {
        self.info.disable_irq(UCRXIE);
        gpio::set_gpio_function(&self.pin);
    }
}

impl embedded_io_async::ErrorType for Uart<'_> {
    type Error = Error;
}

impl embedded_io_async::ErrorType for UartTx<'_> {
    type Error = Error;
}

impl embedded_io_async::ErrorType for UartRx<'_> {
    type Error = Error;
}

impl embedded_io_async::Write for UartTx<'_> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        UartTx::write(self, buf).await?;
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        UartTx::flush(self).await
    }
}

impl embedded_io_async::Write for Uart<'_> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Uart::write(self, buf).await?;
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Uart::flush(self).await
    }
}

impl embedded_io_async::Read for UartRx<'_> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        // Read one character and return, rather than filling the buffer: `Read` is allowed to
        // return short, and blocking until the buffer is full would deadlock a line-based protocol.
        UartRx::read(self, &mut buf[..1]).await?;
        Ok(1)
    }
}

impl embedded_io_async::Read for Uart<'_> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        embedded_io_async::Read::read(&mut self.rx, buf).await
    }
}
