//! The F1xx UART: a USART in asynchronous mode.
//!
//! The USART predates the eUSCI and is simpler in every direction: its registers are bytes rather
//! than words, its enable and interrupt bits live in the chip's special function registers rather
//! than in the peripheral, and its baud rate generator has one modulation byte instead of a
//! prescaler and a lookup table.
//!
//! On a board with the usual 32768 Hz watch crystal, [`PeripheralClock::Aclk`] gives 9600 baud
//! almost exactly and nothing faster. Higher rates need SMCLK, and SMCLK on this family is an
//! untrimmed DCO — see [`crate::clock`], and expect to have to tell the HAL what it really runs at.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};

use super::{Config, ConfigError, DataBits, Error, Parity, StopBits};
use crate::clock::PeripheralClock;
use crate::gpio::{self, AnyPin, Pin};
use crate::peripherals;
use crate::usart::{self, Info};

// Control register.
const SWRST: u8 = 0x01;
const CHAR_8BIT: u8 = 0x10;
const SPB_2: u8 = 0x20;
const PEV: u8 = 0x40;
const PENA: u8 = 0x80;

// Transmit control register.
const TXEPT: u8 = 0x01;
const SSEL_ACLK: u8 = 0x10;
const SSEL_SMCLK: u8 = 0x20;

// Receive control register.
const RXERR: u8 = 0x01;
const BRK: u8 = 0x10;
const OE: u8 = 0x20;
const PE: u8 = 0x40;
const FE: u8 = 0x80;

// Register offsets from the peripheral base. All of these are bytes.
const CTL: u16 = 0x00;
const TCTL: u16 = 0x01;
const RCTL: u16 = 0x02;
const MCTL: u16 = 0x03;
const BR0: u16 = 0x04;
const BR1: u16 = 0x05;
const RXBUF: u16 = 0x06;
const TXBUF: u16 = 0x07;

/// Turn the receive status bits into an error.
///
/// Must be read before `RXBUF`: reading the character clears the status along with the flag.
fn check_rx_error(info: &Info) -> Result<(), Error> {
    let rctl = info.read(RCTL);
    if rctl & RXERR == 0 {
        return Ok(());
    }
    if rctl & FE != 0 {
        Err(Error::Framing)
    } else if rctl & PE != 0 {
        Err(Error::Parity)
    } else if rctl & OE != 0 {
        Err(Error::Overrun)
    } else if rctl & BRK != 0 {
        Err(Error::Break)
    } else {
        Ok(())
    }
}

/// Bit rate generator settings: a 16-bit divider and a modulation byte.
struct BaudConfig {
    div: u16,
    mctl: u8,
}

/// Work out the divider and modulation pattern for `baudrate` from a `clock` Hz bit clock.
///
/// There is no oversampling here and no lookup table. The divider takes the whole number of clocks
/// per bit; the modulation byte then says which of the next eight bit times get one extra clock, so
/// that eight bits together come out to the right length. Bit `i` is set when the running total of
/// the fraction ticks over between bit `i` and bit `i + 1`, which spreads the error as evenly as
/// eight bits allow.
fn calc_baud(clock: u32, baudrate: u32) -> Result<BaudConfig, ConfigError> {
    if baudrate == 0 || clock == 0 {
        return Err(ConfigError::UnachievableBaudrate);
    }

    let div = clock / baudrate;
    if div < 2 || div > 0xFFFF {
        return Err(ConfigError::UnachievableBaudrate);
    }

    let remainder = clock % baudrate;
    let mut mctl = 0u8;
    for i in 0..8u32 {
        if ((i + 1) * remainder) / baudrate > (i * remainder) / baudrate {
            mctl |= 1 << i;
        }
    }

    Ok(BaudConfig { div: div as u16, mctl })
}

trait SealedInstance {
    fn info() -> &'static Info;
}

/// A USART usable as a UART.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static {}

macro_rules! impl_instance {
    ($peri:ident, $info:ident) => {
        impl SealedInstance for peripherals::$peri {
            fn info() -> &'static Info {
                &usart::$info
            }
        }
        impl Instance for peripherals::$peri {}
    };
}

impl_instance!(USART0, INFO_U0);
impl_instance!(USART1, INFO_U1);

/// A pin that can be an instance's transmit line.
#[allow(private_bounds)]
pub trait TxPin<T: Instance>: Pin {}
/// A pin that can be an instance's receive line.
#[allow(private_bounds)]
pub trait RxPin<T: Instance>: Pin {}

impl TxPin<peripherals::USART0> for peripherals::P3_4 {}
impl RxPin<peripherals::USART0> for peripherals::P3_5 {}
impl TxPin<peripherals::USART1> for peripherals::P3_6 {}
impl RxPin<peripherals::USART1> for peripherals::P3_7 {}

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

        // Hand the pins to the USART only once it is configured, so nothing is driven meanwhile.
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
/// The USART has to be held in reset for this, and its module enable in the special function
/// registers turned on afterwards — without that the pins stay disconnected however the port is
/// configured.
fn configure(info: &'static Info, config: Config) -> Result<(), ConfigError> {
    let clocks = crate::clocks().ok_or(ConfigError::ClocksNotInitialized)?;
    let (source_bits, source_hz) = match config.clock_source {
        PeripheralClock::Smclk => (SSEL_SMCLK, clocks.smclk),
        PeripheralClock::Aclk => (SSEL_ACLK, clocks.aclk),
    };
    let baud = calc_baud(source_hz, config.baudrate)?;

    // SYNC stays 0, which is asynchronous mode; MM stays 0, which is idle-line multiprocessor and
    // irrelevant with the wake-up features off.
    let mut ctl = SWRST;
    if config.data_bits == DataBits::Eight {
        ctl |= CHAR_8BIT;
    }
    if config.stop_bits == StopBits::Two {
        ctl |= SPB_2;
    }
    match config.parity {
        Parity::None => {}
        Parity::Even => ctl |= PENA | PEV,
        Parity::Odd => ctl |= PENA,
    }

    info.write(CTL, SWRST);
    info.write(CTL, ctl);
    info.write(TCTL, source_bits);
    info.write(RCTL, 0);
    info.write(BR0, baud.div as u8);
    info.write(BR1, (baud.div >> 8) as u8);
    info.write(MCTL, baud.mctl);

    // Connect the module, then release it from reset. The transmit flag comes up set, meaning the
    // buffer is empty.
    info.set_enabled(info.rx_bit | info.tx_bit, true);
    info.disable_irq(info.rx_bit | info.tx_bit);
    info.modify(CTL, |v| v & !SWRST);

    Ok(())
}

impl<'d> UartTx<'d> {
    /// Send every byte of `buf`, waiting for the hardware between characters.
    pub async fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        for &byte in buf {
            let info = self.info;
            poll_fn(|cx| {
                critical_section::with(|_| {
                    if info.flag(info.tx_bit) {
                        info.write(TXBUF, byte);
                        Poll::Ready(())
                    } else {
                        // Register before unmasking: the handler cannot run until this critical
                        // section ends, so the wakeup cannot be missed.
                        info.waker(1).register(cx.waker());
                        info.enable_irq(info.tx_bit);
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
            while !self.info.flag(self.info.tx_bit) {}
            self.info.write(TXBUF, byte);
        }
        Ok(())
    }

    /// Wait until the last character has left the shift register.
    ///
    /// There is no interrupt for this on the USART, so it spins. At 9600 baud that is about a
    /// millisecond after the last byte.
    pub async fn flush(&mut self) -> Result<(), Error> {
        self.blocking_flush()
    }

    /// Wait until the last character has left the shift register, spinning.
    pub fn blocking_flush(&mut self) -> Result<(), Error> {
        while self.info.read(TCTL) & TXEPT == 0 {}
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
                    if info.flag(info.rx_bit) {
                        // Check the status before reading the buffer: reading it clears both.
                        let res = check_rx_error(info);
                        let byte = info.read(RXBUF);
                        Poll::Ready(res.map(|()| byte))
                    } else {
                        info.waker(0).register(cx.waker());
                        info.enable_irq(info.rx_bit);
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
            while !self.info.flag(self.info.rx_bit) {}
            check_rx_error(self.info)?;
            *slot = self.info.read(RXBUF);
        }
        Ok(())
    }
}

impl<'d> Drop for UartTx<'d> {
    fn drop(&mut self) {
        self.info.disable_irq(self.info.tx_bit);
        self.info.set_enabled(self.info.tx_bit, false);
        gpio::set_gpio_function(&self.pin);
    }
}

impl<'d> Drop for UartRx<'d> {
    fn drop(&mut self) {
        self.info.disable_irq(self.info.rx_bit);
        self.info.set_enabled(self.info.rx_bit, false);
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
