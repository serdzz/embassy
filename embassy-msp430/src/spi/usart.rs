//! The F1xx SPI: a USART in synchronous mode.
//!
//! # Blocking or async
//!
//! Both are provided, and which is faster depends on the bit rate. This chip has no DMA, so an
//! async transfer suspends the task once per byte; that round trip costs more than the eight bit
//! times it hides at 1 MHz, and much less than them at 100 kHz. Reach for [`Spi::blocking_write`]
//! and friends for short, fast transfers, and the async ones when the bus is slow enough that
//! letting the executor sleep is worth it.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embedded_hal::spi::{Phase, Polarity};

use super::{BitOrder, Config, ConfigError, Error};
use crate::clock::PeripheralClock;
use crate::gpio::{self, AnyPin, Pin};
use crate::peripherals;
use crate::usart::{self, Info};

// Control register.
const SWRST: u8 = 0x01;
const MM: u8 = 0x02;
const SYNC: u8 = 0x04;
const CHAR_8BIT: u8 = 0x10;

// Transmit control register.
const TXEPT: u8 = 0x01;
/// Three-pin SPI: `STE` is not used, so the pin stays yours.
const STC: u8 = 0x02;
const SSEL_ACLK: u8 = 0x10;
const SSEL_SMCLK: u8 = 0x20;
const CKPL: u8 = 0x40;
const CKPH: u8 = 0x80;

// Receive control register.
const OE: u8 = 0x20;

// Register offsets from the peripheral base. All of these are bytes.
const CTL: u16 = 0x00;
const TCTL: u16 = 0x01;
const RCTL: u16 = 0x02;
const MCTL: u16 = 0x03;
const BR0: u16 = 0x04;
const BR1: u16 = 0x05;
const RXBUF: u16 = 0x06;
const TXBUF: u16 = 0x07;

trait SealedInstance {
    fn info() -> &'static Info;
}

/// A USART usable as an SPI master.
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

/// A pin that can be an instance's clock output.
#[allow(private_bounds)]
pub trait SckPin<T: Instance>: Pin {}
/// A pin that can be an instance's master-out line (`SIMO`).
#[allow(private_bounds)]
pub trait MosiPin<T: Instance>: Pin {}
/// A pin that can be an instance's master-in line (`SOMI`).
#[allow(private_bounds)]
pub trait MisoPin<T: Instance>: Pin {}

macro_rules! impl_pins {
    ($peri:ident, $sck:ident, $mosi:ident, $miso:ident) => {
        impl SckPin<peripherals::$peri> for peripherals::$sck {}
        impl MosiPin<peripherals::$peri> for peripherals::$mosi {}
        impl MisoPin<peripherals::$peri> for peripherals::$miso {}
    };
}

impl_pins!(USART0, P3_3, P3_1, P3_2);
impl_pins!(USART1, P5_3, P5_1, P5_2);

/// An SPI master.
pub struct Spi<'d> {
    info: &'static Info,
    pins: [Option<Peri<'d, AnyPin>>; 3],
}

impl<'d> Spi<'d> {
    /// Configure `instance` as a full-duplex SPI master.
    pub fn new<T: Instance>(
        _instance: Peri<'d, T>,
        sck: Peri<'d, impl SckPin<T>>,
        mosi: Peri<'d, impl MosiPin<T>>,
        miso: Peri<'d, impl MisoPin<T>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::build::<T>(sck.into(), mosi.into(), Some(miso.into()), config)
    }

    /// Configure `instance` as a transmit-only SPI master, leaving the `SOMI` pin free.
    ///
    /// Reads return zeroes, since nothing is wired to shift data back in.
    pub fn new_txonly<T: Instance>(
        _instance: Peri<'d, T>,
        sck: Peri<'d, impl SckPin<T>>,
        mosi: Peri<'d, impl MosiPin<T>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::build::<T>(sck.into(), mosi.into(), None, config)
    }

    fn build<T: Instance>(
        sck: Peri<'d, AnyPin>,
        mosi: Peri<'d, AnyPin>,
        miso: Option<Peri<'d, AnyPin>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let info = T::info();

        let clocks = crate::clocks().ok_or(ConfigError::ClocksNotInitialized)?;
        let (source_bits, source_hz) = match config.clock_source {
            PeripheralClock::Smclk => (SSEL_SMCLK, clocks.smclk),
            PeripheralClock::Aclk => (SSEL_ACLK, clocks.aclk),
        };
        if config.frequency == 0 || config.frequency > source_hz {
            return Err(ConfigError::UnachievableFrequency);
        }
        let div = (source_hz / config.frequency).min(0xFFFF) as u16;

        if config.bit_order == BitOrder::LsbFirst {
            // The USART has no bit-order control: it is most significant bit first, always.
            return Err(ConfigError::UnsupportedBitOrder);
        }

        let ctl = SWRST | SYNC | MM | CHAR_8BIT;

        let mut tctl = source_bits | STC;
        if config.mode.polarity == Polarity::IdleHigh {
            tctl |= CKPL;
        }
        // CKPH is the inverse of the usual CPHA: the hardware bit says "capture on the first edge",
        // while CPHA = 0 is the mode that captures on the first edge.
        if config.mode.phase == Phase::CaptureOnFirstTransition {
            tctl |= CKPH;
        }

        info.write(CTL, SWRST);
        info.write(CTL, ctl);
        info.write(TCTL, tctl);
        info.write(RCTL, 0);
        info.write(BR0, div as u8);
        info.write(BR1, (div >> 8) as u8);
        // Modulation is a UART idea; in synchronous mode it has to be zero.
        info.write(MCTL, 0);

        info.set_enabled(info.spi_enable_bit, true);
        info.disable_irq(info.rx_bit | info.tx_bit);
        info.modify(CTL, |v| v & !SWRST);

        // Hand the pins over only once the module is driving sensible levels.
        gpio::set_alternate1(&sck);
        gpio::set_alternate1(&mosi);
        if let Some(miso) = &miso {
            gpio::set_alternate1(miso);
        }

        Ok(Self {
            info,
            pins: [Some(sck), Some(mosi), miso],
        })
    }

    fn check_overrun(&self) -> Result<(), Error> {
        if self.info.read(RCTL) & OE != 0 {
            Err(Error::Overrun)
        } else {
            Ok(())
        }
    }

    /// Shift one byte out and the simultaneously received one in, spinning.
    fn blocking_xfer(&mut self, tx: u8) -> Result<u8, Error> {
        let info = self.info;
        while !info.flag(info.tx_bit) {}
        info.write(TXBUF, tx);
        while !info.flag(info.rx_bit) {}
        self.check_overrun()?;
        Ok(info.read(RXBUF))
    }

    /// Shift one byte out and the simultaneously received one in, suspending in between.
    async fn xfer(&mut self, tx: u8) -> Result<u8, Error> {
        let info = self.info;

        poll_fn(|cx| {
            critical_section::with(|_| {
                if info.flag(info.tx_bit) {
                    info.write(TXBUF, tx);
                    Poll::Ready(())
                } else {
                    // Register before unmasking: the handler cannot run until this critical
                    // section ends, so the wakeup cannot be missed.
                    info.waker(0).register(cx.waker());
                    info.enable_irq(info.tx_bit);
                    Poll::Pending
                }
            })
        })
        .await;

        let byte = poll_fn(|cx| {
            critical_section::with(|_| {
                if info.flag(info.rx_bit) {
                    Poll::Ready(info.read(RXBUF))
                } else {
                    info.waker(0).register(cx.waker());
                    info.enable_irq(info.rx_bit);
                    Poll::Pending
                }
            })
        })
        .await;

        self.check_overrun()?;
        Ok(byte)
    }

    /// Send `data`, discarding whatever comes back.
    pub fn blocking_write(&mut self, data: &[u8]) -> Result<(), Error> {
        for &byte in data {
            self.blocking_xfer(byte)?;
        }
        Ok(())
    }

    /// Clock `data.len()` bytes in, sending zeroes.
    pub fn blocking_read(&mut self, data: &mut [u8]) -> Result<(), Error> {
        for slot in data.iter_mut() {
            *slot = self.blocking_xfer(0)?;
        }
        Ok(())
    }

    /// Send `write` and collect the simultaneously received bytes into `read`.
    ///
    /// The transfer runs for as long as the longer of the two: extra received bytes are dropped,
    /// and zeroes are sent once `write` runs out.
    pub fn blocking_transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        let len = read.len().max(write.len());
        for i in 0..len {
            let out = write.get(i).copied().unwrap_or(0);
            let byte = self.blocking_xfer(out)?;
            if let Some(slot) = read.get_mut(i) {
                *slot = byte;
            }
        }
        Ok(())
    }

    /// Send `data` and overwrite it with what came back.
    pub fn blocking_transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), Error> {
        for slot in data.iter_mut() {
            *slot = self.blocking_xfer(*slot)?;
        }
        Ok(())
    }

    /// Wait for the shift register to drain, spinning.
    pub fn blocking_flush(&mut self) -> Result<(), Error> {
        while self.info.read(TCTL) & TXEPT == 0 {}
        Ok(())
    }

    /// Send `data`, discarding whatever comes back.
    pub async fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        for &byte in data {
            self.xfer(byte).await?;
        }
        Ok(())
    }

    /// Clock `data.len()` bytes in, sending zeroes.
    pub async fn read(&mut self, data: &mut [u8]) -> Result<(), Error> {
        for i in 0..data.len() {
            data[i] = self.xfer(0).await?;
        }
        Ok(())
    }

    /// Send `write` and collect the simultaneously received bytes into `read`.
    pub async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        let len = read.len().max(write.len());
        for i in 0..len {
            let out = write.get(i).copied().unwrap_or(0);
            let byte = self.xfer(out).await?;
            if let Some(slot) = read.get_mut(i) {
                *slot = byte;
            }
        }
        Ok(())
    }

    /// Send `data` and overwrite it with what came back.
    pub async fn transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), Error> {
        for i in 0..data.len() {
            data[i] = self.xfer(data[i]).await?;
        }
        Ok(())
    }

    /// Wait for the shift register to drain.
    pub async fn flush(&mut self) -> Result<(), Error> {
        self.blocking_flush()
    }
}

impl<'d> Drop for Spi<'d> {
    fn drop(&mut self) {
        self.info.disable_irq(self.info.rx_bit | self.info.tx_bit);
        // Park the module in reset and disconnect it, so it stops driving the pins before they go
        // back to being GPIO.
        self.info.modify(CTL, |v| v | SWRST);
        self.info.set_enabled(self.info.spi_enable_bit, false);
        for pin in self.pins.iter().flatten() {
            gpio::set_gpio_function(pin);
        }
    }
}

impl embedded_hal::spi::ErrorType for Spi<'_> {
    type Error = Error;
}

impl embedded_hal::spi::SpiBus<u8> for Spi<'_> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.blocking_read(words)
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.blocking_write(words)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.blocking_transfer(read, write)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.blocking_transfer_in_place(words)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.blocking_flush()
    }
}

impl embedded_hal_async::spi::SpiBus<u8> for Spi<'_> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        Spi::read(self, words).await
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        Spi::write(self, words).await
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        Spi::transfer(self, read, write).await
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        Spi::transfer_in_place(self, words).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Spi::flush(self).await
    }
}
