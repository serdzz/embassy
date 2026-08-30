//! The FR2xx SPI: an eUSCI in SPI master mode.
//!
//! Three-wire master only: chip select is left to you as an ordinary [`Output`](crate::gpio::Output),
//! which is what [`embedded_hal::spi::SpiDevice`] implementations expect anyway.
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
use crate::eusci::{self, Info};
use crate::gpio::{self, AnyPin, Pin, PinFunction};
use crate::peripherals;

// Control word 0, SPI mode.
const UCSWRST: u16 = 0x0001;
const UCSSEL_ACLK: u16 = 0x0040;
const UCSSEL_SMCLK: u16 = 0x0080;
const UCSYNC: u16 = 0x0100;
const UCMST: u16 = 0x0800;
const UCMSB: u16 = 0x2000;
const UCCKPL: u16 = 0x4000;
const UCCKPH: u16 = 0x8000;

// Status word.
const UCBUSY: u16 = 0x0001;
const UCOE: u16 = 0x0020;

// Interrupt enable and flags. The bit positions are the same for eUSCI_A and eUSCI_B; only the
// register offsets differ, and those come from `Info`.
const UCRXIE: u16 = 0x0001;
const UCTXIE: u16 = 0x0002;
const UCRXIFG: u16 = 0x0001;
const UCTXIFG: u16 = 0x0002;

// Register offsets from the peripheral base.
const CTLW0: u16 = 0x00;
const BRW: u16 = 0x06;
const RXBUF: u16 = 0x0c;
const TXBUF: u16 = 0x0e;

/// eUSCI_A puts its status register at 0x0A, eUSCI_B at 0x08.
const STATW_A: u16 = 0x0a;
const STATW_B: u16 = 0x08;

trait SealedInstance {
    fn info() -> &'static Info;
    /// Offset of the status register, which differs between the A and B modules.
    fn statw() -> u16;
}

/// An eUSCI module usable as an SPI master.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static {}

macro_rules! impl_instance {
    ($peri:ident, $info:ident, $statw:expr) => {
        impl SealedInstance for peripherals::$peri {
            fn info() -> &'static Info {
                &eusci::$info
            }
            fn statw() -> u16 {
                $statw
            }
        }
        impl Instance for peripherals::$peri {}
    };
}

impl_instance!(EUSCI_A0, INFO_A0, STATW_A);
impl_instance!(EUSCI_A1, INFO_A1, STATW_A);
#[cfg(feature = "_fr504x_604x")]
impl_instance!(EUSCI_A2, INFO_A2, STATW_A);
#[cfg(feature = "_fr504x_604x")]
impl_instance!(EUSCI_A3, INFO_A3, STATW_A);
impl_instance!(EUSCI_B0, INFO_B0, STATW_B);
impl_instance!(EUSCI_B1, INFO_B1, STATW_B);

/// A pin that can be an instance's clock output.
///
/// `ALTERNATE` says which of the pin's alternate functions this eUSCI is — per pin, because the
/// same module is not always the same alternate on every pin that can carry it.
#[allow(private_bounds)]
pub trait SckPin<T: Instance>: Pin {
    /// Which alternate function selects this eUSCI on this pin.
    #[doc(hidden)]
    const ALTERNATE: PinFunction = PinFunction::Alternate1;
}
/// A pin that can be an instance's master-out line (`SIMO`). See [`SckPin`] on `ALTERNATE`.
#[allow(private_bounds)]
pub trait MosiPin<T: Instance>: Pin {
    /// Which alternate function selects this eUSCI on this pin.
    #[doc(hidden)]
    const ALTERNATE: PinFunction = PinFunction::Alternate1;
}
/// A pin that can be an instance's master-in line (`SOMI`). See [`SckPin`] on `ALTERNATE`.
#[allow(private_bounds)]
pub trait MisoPin<T: Instance>: Pin {
    /// Which alternate function selects this eUSCI on this pin.
    #[doc(hidden)]
    const ALTERNATE: PinFunction = PinFunction::Alternate1;
}

macro_rules! impl_pins {
    ($peri:ident, $sck:ident, $mosi:ident, $miso:ident) => {
        impl_pins!($peri, $sck, $mosi, $miso, PinFunction::Alternate1);
    };
    ($peri:ident, $sck:ident, $mosi:ident, $miso:ident, $alt:expr) => {
        impl SckPin<peripherals::$peri> for peripherals::$sck {
            const ALTERNATE: PinFunction = $alt;
        }
        impl MosiPin<peripherals::$peri> for peripherals::$mosi {
            const ALTERNATE: PinFunction = $alt;
        }
        impl MisoPin<peripherals::$peri> for peripherals::$miso {
            const ALTERNATE: PinFunction = $alt;
        }
    };
}

#[cfg(feature = "msp430fr2355")]
mod pins {
    use super::*;

    impl_pins!(EUSCI_A0, P1_5, P1_7, P1_6);
    impl_pins!(EUSCI_A1, P4_1, P4_3, P4_2);
    impl_pins!(EUSCI_B0, P1_1, P1_2, P1_3);
    impl_pins!(EUSCI_B1, P4_5, P4_6, P4_7);
}

/// Which pins carry which eUSCI as an SPI on the MSP430FR6043.
///
/// Only one set of pins per instance, chosen so that all three lines of a set sit on the same
/// alternate — the parts have more mappings than this, but a set split across two alternates is a
/// wiring trap rather than a feature. See the note in `uart::TxPin`: the alternates are derived
/// from the datasheet's pinout ordering and want checking against Table 7-1.
#[cfg(feature = "_fr504x_604x")]
mod pins {
    use super::*;

    // P4.1/UCA0CLK, P4.3/UCA0SIMO, P4.4/UCA0SOMI
    impl_pins!(EUSCI_A0, P4_1, P4_3, P4_4);
    // P1.0/UCA1CLK, P1.2/UCA1SIMO, P1.3/UCA1SOMI
    impl_pins!(EUSCI_A1, P1_0, P1_2, P1_3);
    // P5.2/TB0.2/UCA2CLK, P5.0/TB0.0/UCA2SIMO, P5.1/TB0.1/UCA2SOMI
    impl_pins!(EUSCI_A2, P5_2, P5_0, P5_1, PinFunction::Alternate2);
    // P1.7/USSTRG/UCA3CLK, P2.0/UCA1CLK/UCA3SIMO, P2.1/UCA1STE/UCA3SOMI
    impl_pins!(EUSCI_A3, P1_7, P2_0, P2_1, PinFunction::Alternate2);
    // P5.4/TA0.0/UCB1CLK, P5.5/TA4.1/UCB1SIMO, P5.6/TB0OUTH/UCB1SOMI
    impl_pins!(EUSCI_B1, P5_4, P5_5, P5_6, PinFunction::Alternate2);

    // UCB0 is the exception only in that all three of its lines are second alternates while the
    // macro's default is the first, so it cannot go through the macro.
    // P1.5/TB0.5/UCB0CLK
    impl SckPin<peripherals::EUSCI_B0> for peripherals::P1_5 {
        const ALTERNATE: PinFunction = PinFunction::Alternate2;
    }
    // P1.6/UCA3STE/UCB0SIMO
    impl MosiPin<peripherals::EUSCI_B0> for peripherals::P1_6 {
        const ALTERNATE: PinFunction = PinFunction::Alternate2;
    }
    // P1.7/USSTRG/UCA3CLK/UCB0SOMI, RGC64 pin 24. Second alternate, not third: `USSTRG` is an
    // independent function rather than a `SEL` encoding. See the same correction in `i2c`.
    impl MisoPin<peripherals::EUSCI_B0> for peripherals::P1_7 {
        const ALTERNATE: PinFunction = PinFunction::Alternate2;
    }
}

/// An SPI master.
pub struct Spi<'d> {
    info: &'static Info,
    statw: u16,
    pins: [Option<Peri<'d, AnyPin>>; 3],
}

impl<'d> Spi<'d> {
    /// Configure `instance` as a full-duplex SPI master.
    pub fn new<T: Instance, S: SckPin<T>, M: MosiPin<T>, I: MisoPin<T>>(
        _instance: Peri<'d, T>,
        sck: Peri<'d, S>,
        mosi: Peri<'d, M>,
        miso: Peri<'d, I>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::build::<T>(
            (sck.into(), S::ALTERNATE),
            (mosi.into(), M::ALTERNATE),
            Some((miso.into(), I::ALTERNATE)),
            config,
        )
    }

    /// Configure `instance` as a transmit-only SPI master, leaving the `SOMI` pin free.
    ///
    /// Reads return zeroes, since nothing is wired to shift data back in.
    pub fn new_txonly<T: Instance, S: SckPin<T>, M: MosiPin<T>>(
        _instance: Peri<'d, T>,
        sck: Peri<'d, S>,
        mosi: Peri<'d, M>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::build::<T>(
            (sck.into(), S::ALTERNATE),
            (mosi.into(), M::ALTERNATE),
            None,
            config,
        )
    }

    /// Each pin arrives with the alternate function that reaches this eUSCI on it, since the type
    /// that knew has been erased by now.
    fn build<T: Instance>(
        sck: (Peri<'d, AnyPin>, PinFunction),
        mosi: (Peri<'d, AnyPin>, PinFunction),
        miso: Option<(Peri<'d, AnyPin>, PinFunction)>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let (sck, sck_alt) = sck;
        let (mosi, mosi_alt) = mosi;
        let (miso, miso_alt) = match miso {
            Some((pin, alt)) => (Some(pin), alt),
            None => (None, PinFunction::Alternate1),
        };
        let info = T::info();

        let clocks = crate::clocks().ok_or(ConfigError::ClocksNotInitialized)?;
        let (source_bits, source_hz) = match config.clock_source {
            PeripheralClock::Smclk => (UCSSEL_SMCLK, clocks.smclk),
            PeripheralClock::Aclk => (UCSSEL_ACLK, clocks.aclk),
        };
        if config.frequency == 0 || config.frequency > source_hz {
            return Err(ConfigError::UnachievableFrequency);
        }
        let brw = (source_hz / config.frequency).min(0xFFFF) as u16;

        // UCMODE stays 00, three-pin SPI: STE is not used, so the pin stays yours.
        let mut ctlw0 = UCSWRST | UCSYNC | UCMST | source_bits;
        if config.bit_order == BitOrder::MsbFirst {
            ctlw0 |= UCMSB;
        }
        if config.mode.polarity == Polarity::IdleHigh {
            ctlw0 |= UCCKPL;
        }
        // UCCKPH is the inverse of the usual CPHA: the hardware bit says "capture on the first
        // edge", while CPHA = 0 is the mode that captures on the first edge.
        if config.mode.phase == Phase::CaptureOnFirstTransition {
            ctlw0 |= UCCKPH;
        }

        // The bit rate and most control bits are only latched while UCSWRST is set.
        info.write(CTLW0, UCSWRST);
        info.write(CTLW0, ctlw0);
        info.write(BRW, brw);
        info.write(info.ie_off, 0);
        info.write(info.ifg_off, 0);
        info.write(CTLW0, ctlw0 & !UCSWRST);

        // Hand the pins over only once the module is driving sensible levels.
        gpio::set_alternate(&sck, sck_alt);
        gpio::set_alternate(&mosi, mosi_alt);
        if let Some(miso) = &miso {
            gpio::set_alternate(miso, miso_alt);
        }

        Ok(Self {
            info,
            statw: T::statw(),
            pins: [Some(sck), Some(mosi), miso],
        })
    }

    fn check_overrun(&self) -> Result<(), Error> {
        if self.info.read(self.statw) & UCOE != 0 {
            Err(Error::Overrun)
        } else {
            Ok(())
        }
    }

    /// Shift one byte out and the simultaneously received one in, spinning.
    fn blocking_xfer(&mut self, tx: u8) -> Result<u8, Error> {
        let info = self.info;
        while info.ifg() & UCTXIFG == 0 {}
        info.write(TXBUF, tx as u16);
        while info.ifg() & UCRXIFG == 0 {}
        self.check_overrun()?;
        Ok(info.read(RXBUF) as u8)
    }

    /// Shift one byte out and the simultaneously received one in, suspending in between.
    async fn xfer(&mut self, tx: u8) -> Result<u8, Error> {
        let info = self.info;

        poll_fn(|cx| {
            critical_section::with(|_| {
                if info.ifg() & UCTXIFG != 0 {
                    info.write(TXBUF, tx as u16);
                    Poll::Ready(())
                } else {
                    // Register before unmasking: the handler cannot run until this critical
                    // section ends, so the wakeup cannot be missed.
                    info.waker(0).register(cx.waker());
                    info.enable_irq(UCTXIE);
                    Poll::Pending
                }
            })
        })
        .await;

        let byte = poll_fn(|cx| {
            critical_section::with(|_| {
                if info.ifg() & UCRXIFG != 0 {
                    Poll::Ready(info.read(RXBUF) as u8)
                } else {
                    info.waker(0).register(cx.waker());
                    info.enable_irq(UCRXIE);
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
        while self.info.read(self.statw) & UCBUSY != 0 {}
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
        self.info.disable_irq(UCRXIE | UCTXIE);
        // Park the module in reset so it stops driving the pins before they go back to GPIO.
        self.info.modify(CTLW0, |v| v | UCSWRST);
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
