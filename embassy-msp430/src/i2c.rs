//! Inter-integrated circuit (eUSCI_B in I2C master mode).
//!
//! Single master, 7-bit addressing.
//!
//! # Transaction shapes
//!
//! [`embedded_hal::i2c::I2c::transaction`] is implemented for the shapes device drivers actually
//! use: any number of writes, optionally followed by any number of reads, with a repeated start at
//! the changeover. Consecutive operations of the same direction are merged, as the trait requires.
//!
//! Going back from reading to writing within one transaction returns
//! [`Error::UnsupportedTransaction`] rather than guessing at a sequence this driver has not been
//! able to verify against hardware.
//!
//! # Hanging buses
//!
//! A slave that holds SCL low stalls the master forever. The eUSCI can catch that in hardware, and
//! [`Config::clock_low_timeout`] leaves it on by default, turning the stall into
//! [`Error::Timeout`] after roughly 31 ms.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embedded_hal::i2c::Operation;

use crate::clock::PeripheralClock;
use crate::eusci::{self, Info};
use crate::gpio::{self, AnyPin, Pin, PinFunction};
use crate::peripherals;

// Control word 0, I2C mode.
const UCSWRST: u16 = 0x0001;
const UCTXSTT: u16 = 0x0002;
const UCTXSTP: u16 = 0x0004;
const UCTR: u16 = 0x0010;
const UCSSEL_ACLK: u16 = 0x0040;
const UCSSEL_SMCLK: u16 = 0x0080;
const UCSYNC: u16 = 0x0100;
const UCMODE_I2C: u16 = 0x0600;
const UCMST: u16 = 0x0800;

// Control word 1.
const UCCLTO_31MS: u16 = 0x0080;

// Interrupt flags, and the enable bits at the same positions.
const UCRXIFG0: u16 = 0x0001;
const UCTXIFG0: u16 = 0x0002;
const UCSTPIFG: u16 = 0x0008;
const UCALIFG: u16 = 0x0100;
const UCNACKIFG: u16 = 0x0200;
const UCCLTOIFG: u16 = 0x0800;

/// Everything that ends a wait badly.
const FAULTS: u16 = UCALIFG | UCNACKIFG | UCCLTOIFG;

// Register offsets from the peripheral base.
const CTLW0: u16 = 0x00;
const CTLW1: u16 = 0x02;
const BRW: u16 = 0x06;
const RXBUF: u16 = 0x0c;
const TXBUF: u16 = 0x0e;
const I2CSA: u16 = 0x20;

/// I2C configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// Bus clock in Hz. Rounded down to what the integer divider can produce.
    pub frequency: u32,
    /// Clock the bit rate generator runs from.
    pub clock_source: PeripheralClock,
    /// Let the hardware give up when a slave holds SCL low for about 31 ms.
    pub clock_low_timeout: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 100_000,
            clock_source: PeripheralClock::default(),
            clock_low_timeout: true,
        }
    }
}

/// Reasons a [`Config`] cannot be applied.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// The divider cannot reach this bus clock from the selected source.
    UnachievableFrequency,
    /// [`crate::init`] has not run, so the clock frequencies are unknown.
    ClocksNotInitialized,
}

/// I2C errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Nobody acknowledged the address, or a slave stopped acknowledging data.
    Nack,
    /// Another master won the bus.
    ArbitrationLoss,
    /// A slave held SCL low past the hardware timeout.
    Timeout,
    /// The address does not fit 7 bits.
    AddressTooLarge,
    /// This driver does not implement this sequence of operations. See the module docs.
    UnsupportedTransaction,
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource};
        match self {
            Error::Nack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Error::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            _ => ErrorKind::Other,
        }
    }
}

trait SealedInstance {
    fn info() -> &'static Info;
}

/// An eUSCI_B module usable as an I2C master.
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

impl_instance!(EUSCI_B0, INFO_B0);
impl_instance!(EUSCI_B1, INFO_B1);

/// A pin that can be an instance's clock line.
///
/// `ALTERNATE` says which of the pin's alternate functions this eUSCI is — per pin, because the
/// same module is not always the same alternate on every pin that can carry it.
#[allow(private_bounds)]
pub trait SclPin<T: Instance>: Pin {
    /// Which alternate function selects this eUSCI on this pin.
    #[doc(hidden)]
    const ALTERNATE: PinFunction = PinFunction::Alternate1;
}
/// A pin that can be an instance's data line. See [`SclPin`] on `ALTERNATE`.
#[allow(private_bounds)]
pub trait SdaPin<T: Instance>: Pin {
    /// Which alternate function selects this eUSCI on this pin.
    #[doc(hidden)]
    const ALTERNATE: PinFunction = PinFunction::Alternate1;
}

#[cfg(feature = "msp430fr2355")]
mod pins {
    use super::*;

    impl SclPin<peripherals::EUSCI_B0> for peripherals::P1_3 {}
    impl SdaPin<peripherals::EUSCI_B0> for peripherals::P1_2 {}
    impl SclPin<peripherals::EUSCI_B1> for peripherals::P4_7 {}
    impl SdaPin<peripherals::EUSCI_B1> for peripherals::P4_6 {}
}

/// Which pins carry I2C on the MSP430FR6043 and MSP430FR5043.
///
/// Checked against SLASEF5B Table 9-27, which is the table that settles it: Table 7-1 lists a pin's
/// signals with the port name floating in the middle of the block, so reading alternates off it is
/// how the tertiary/secondary mistake below got made in the first place.
#[cfg(feature = "_fr504x_604x")]
mod pins {
    use super::*;

    // P1.7/USSTRG/UCA3CLK/UCB0SOMI/UCB0SCL, RGC64 pin 24.
    //
    // Second alternate, the same as its data line. It was third here until the datasheet was
    // actually opened: `USSTRG` looks like an alternate function and is not one -- SLASEF5B
    // Table 9-27 lists it as an independent function with no `SEL` encoding of its own, which
    // leaves `UCA3CLK` primary and `UCB0SOMI/UCB0SCL` secondary. Selecting the third alternate
    // instead gives the row Table 9-27 marks "N/A -- internally tied to DVSS", so SCL would have
    // been held low and no I2C transfer could ever have started.
    impl SclPin<peripherals::EUSCI_B0> for peripherals::P1_7 {
        const ALTERNATE: PinFunction = PinFunction::Alternate2;
    }
    // P1.6/UCA3STE/UCB0SIMO/UCB0SDA, RGC64 pin 23. Second alternate -- SLASEF5B Table 9-27.
    impl SdaPin<peripherals::EUSCI_B0> for peripherals::P1_6 {
        const ALTERNATE: PinFunction = PinFunction::Alternate2;
    }
    // P5.6/TB0OUTH/UCB1SOMI/UCB1SCL
    impl SclPin<peripherals::EUSCI_B1> for peripherals::P5_6 {
        const ALTERNATE: PinFunction = PinFunction::Alternate2;
    }
    // P5.5/TA4.1/UCB1SIMO/UCB1SDA
    impl SdaPin<peripherals::EUSCI_B1> for peripherals::P5_5 {
        const ALTERNATE: PinFunction = PinFunction::Alternate2;
    }
}

/// An I2C master.
pub struct I2c<'d> {
    info: &'static Info,
    pins: [Peri<'d, AnyPin>; 2],
}

/// How a transaction splits into a write phase and a read phase.
struct Phases {
    writes: usize,
    reads: usize,
}

/// Check that `ops` is a shape this driver implements, and say where the direction changes.
fn split_phases(ops: &[Operation<'_>]) -> Result<Phases, Error> {
    let writes = ops.iter().take_while(|op| matches!(op, Operation::Write(_))).count();
    // Everything after the leading writes has to be a read; a second direction change is what we
    // do not implement.
    if ops[writes..].iter().any(|op| matches!(op, Operation::Write(_))) {
        return Err(Error::UnsupportedTransaction);
    }
    Ok(Phases {
        writes,
        reads: ops.len() - writes,
    })
}

impl<'d> I2c<'d> {
    /// Configure `instance` as an I2C master.
    ///
    /// Both lines need external pull-ups; the eUSCI drives them open-drain and does not pull up.
    pub fn new<T: Instance, C: SclPin<T>, D: SdaPin<T>>(
        _instance: Peri<'d, T>,
        scl: Peri<'d, C>,
        sda: Peri<'d, D>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let info = T::info();
        // Read off the concrete pin types before they are erased.
        let scl_alt = C::ALTERNATE;
        let sda_alt = D::ALTERNATE;
        let scl: Peri<'d, AnyPin> = scl.into();
        let sda: Peri<'d, AnyPin> = sda.into();

        let clocks = crate::clocks().ok_or(ConfigError::ClocksNotInitialized)?;
        let (source_bits, source_hz) = match config.clock_source {
            PeripheralClock::Smclk => (UCSSEL_SMCLK, clocks.smclk),
            PeripheralClock::Aclk => (UCSSEL_ACLK, clocks.aclk),
        };
        if config.frequency == 0 || config.frequency > source_hz {
            return Err(ConfigError::UnachievableFrequency);
        }
        let brw = (source_hz / config.frequency).min(0xFFFF) as u16;

        let ctlw0 = UCSWRST | UCMODE_I2C | UCSYNC | UCMST | source_bits;
        let ctlw1 = if config.clock_low_timeout { UCCLTO_31MS } else { 0 };

        // Held in reset: most control bits, and the bit rate, are only latched while UCSWRST is set.
        info.write(CTLW0, UCSWRST);
        info.write(CTLW0, ctlw0);
        info.write(CTLW1, ctlw1);
        info.write(BRW, brw);
        info.write(info.ie_off, 0);
        info.write(info.ifg_off, 0);
        info.write(CTLW0, ctlw0 & !UCSWRST);

        gpio::set_alternate(&scl, scl_alt);
        gpio::set_alternate(&sda, sda_alt);

        Ok(Self { info, pins: [scl, sda] })
    }

    /// Turn whichever fault flag is up into an error, and leave the bus in a usable state.
    fn fault(&mut self, flags: u16) -> Error {
        // A NACK leaves the bus held; a stop is the only way to release it.
        self.info.clear_flags(FAULTS);
        self.info.modify(CTLW0, |v| v | UCTXSTP);
        if flags & UCNACKIFG != 0 {
            Error::Nack
        } else if flags & UCALIFG != 0 {
            Error::ArbitrationLoss
        } else {
            Error::Timeout
        }
    }

    /// Spin until one of `mask` or a fault shows up.
    fn blocking_wait(&mut self, mask: u16) -> Result<u16, Error> {
        loop {
            let ifg = self.info.ifg();
            if ifg & FAULTS != 0 {
                return Err(self.fault(ifg));
            }
            if ifg & mask != 0 {
                return Ok(ifg & mask);
            }
        }
    }

    /// Suspend until one of `mask` or a fault shows up.
    async fn wait(&mut self, mask: u16) -> Result<u16, Error> {
        let info = self.info;
        let ifg = poll_fn(|cx| {
            critical_section::with(|_| {
                let ifg = info.ifg();
                if ifg & (mask | FAULTS) != 0 {
                    Poll::Ready(ifg)
                } else {
                    // Register before unmasking: the handler cannot run until this critical
                    // section ends, so the wakeup cannot be missed.
                    info.waker(0).register(cx.waker());
                    info.enable_irq(mask | FAULTS);
                    Poll::Pending
                }
            })
        })
        .await;

        if ifg & FAULTS != 0 {
            Err(self.fault(ifg))
        } else {
            Ok(ifg & mask)
        }
    }

    /// Address the slave and start a transfer in the given direction.
    ///
    /// Setting `UCTXSTT` while a transfer is already running is how a repeated start is made, so
    /// this covers both the first phase and the changeover.
    fn start(&mut self, address: u8, read: bool) -> Result<(), Error> {
        if address > 0x7f {
            return Err(Error::AddressTooLarge);
        }
        self.info.write(I2CSA, address as u16);
        self.info.clear_flags(FAULTS | UCSTPIFG);
        self.info.modify(CTLW0, |v| {
            let v = if read { v & !UCTR } else { v | UCTR };
            v | UCTXSTT
        });
        Ok(())
    }

    /// Ask for a stop, and wait until it has actually gone out.
    fn blocking_stop(&mut self) -> Result<(), Error> {
        self.info.modify(CTLW0, |v| v | UCTXSTP);
        self.blocking_wait(UCSTPIFG)?;
        self.info.clear_flags(UCSTPIFG);
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Error> {
        self.info.modify(CTLW0, |v| v | UCTXSTP);
        self.wait(UCSTPIFG).await?;
        self.info.clear_flags(UCSTPIFG);
        Ok(())
    }

    /// Send a single byte to `EUSCI_B` and wait for the buffer to free up again.
    fn blocking_write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        for &byte in bytes {
            self.blocking_wait(UCTXIFG0)?;
            self.info.write(TXBUF, byte as u16);
        }
        Ok(())
    }

    async fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        for &byte in bytes {
            self.wait(UCTXIFG0).await?;
            self.info.write(TXBUF, byte as u16);
        }
        Ok(())
    }

    /// Receive into `buf`. `last` says whether the final byte of `buf` ends the whole transaction,
    /// in which case the stop has to be requested before that byte is clocked in.
    fn blocking_read_bytes(&mut self, buf: &mut [u8], last: bool) -> Result<(), Error> {
        let n = buf.len();
        for i in 0..n {
            if last && i + 1 == n {
                if n == 1 {
                    // With a single byte there is no earlier byte to hide the request behind: the
                    // stop can only be asked for once the address phase is over. Bounded by one
                    // address phase, so this spin is short even at 100 kHz.
                    while self.info.read(CTLW0) & UCTXSTT != 0 {
                        let ifg = self.info.ifg();
                        if ifg & FAULTS != 0 {
                            return Err(self.fault(ifg));
                        }
                    }
                }
                self.info.modify(CTLW0, |v| v | UCTXSTP);
            }
            self.blocking_wait(UCRXIFG0)?;
            buf[i] = self.info.read(RXBUF) as u8;
        }
        Ok(())
    }

    async fn read_bytes(&mut self, buf: &mut [u8], last: bool) -> Result<(), Error> {
        let n = buf.len();
        for i in 0..n {
            if last && i + 1 == n {
                if n == 1 {
                    while self.info.read(CTLW0) & UCTXSTT != 0 {
                        let ifg = self.info.ifg();
                        if ifg & FAULTS != 0 {
                            return Err(self.fault(ifg));
                        }
                    }
                }
                self.info.modify(CTLW0, |v| v | UCTXSTP);
            }
            self.wait(UCRXIFG0).await?;
            buf[i] = self.info.read(RXBUF) as u8;
        }
        Ok(())
    }

    /// Run `ops` as one transaction, spinning between bytes.
    pub fn blocking_transaction(&mut self, address: u8, ops: &mut [Operation<'_>]) -> Result<(), Error> {
        if ops.is_empty() {
            return Ok(());
        }
        let phases = split_phases(ops)?;
        let (write_ops, read_ops) = ops.split_at_mut(phases.writes);

        if phases.writes > 0 {
            self.start(address, false)?;
            for op in write_ops.iter_mut() {
                if let Operation::Write(bytes) = op {
                    self.blocking_write_bytes(bytes)?;
                }
            }
            if phases.reads == 0 {
                // Let the last byte reach the shift register before asking for the stop.
                self.blocking_wait(UCTXIFG0)?;
                return self.blocking_stop();
            }
        }

        // Repeated start if a write phase came first, plain start otherwise.
        self.start(address, true)?;
        let last_idx = phases.reads - 1;
        for (i, op) in read_ops.iter_mut().enumerate() {
            if let Operation::Read(buf) = op {
                self.blocking_read_bytes(buf, i == last_idx)?;
            }
        }
        self.blocking_wait(UCSTPIFG)?;
        self.info.clear_flags(UCSTPIFG);
        Ok(())
    }

    /// Run `ops` as one transaction.
    pub async fn transaction(&mut self, address: u8, ops: &mut [Operation<'_>]) -> Result<(), Error> {
        if ops.is_empty() {
            return Ok(());
        }
        let phases = split_phases(ops)?;
        let (write_ops, read_ops) = ops.split_at_mut(phases.writes);

        if phases.writes > 0 {
            self.start(address, false)?;
            for op in write_ops.iter_mut() {
                if let Operation::Write(bytes) = op {
                    self.write_bytes(bytes).await?;
                }
            }
            if phases.reads == 0 {
                self.wait(UCTXIFG0).await?;
                return self.stop().await;
            }
        }

        self.start(address, true)?;
        let last_idx = phases.reads - 1;
        for (i, op) in read_ops.iter_mut().enumerate() {
            if let Operation::Read(buf) = op {
                self.read_bytes(buf, i == last_idx).await?;
            }
        }
        self.wait(UCSTPIFG).await?;
        self.info.clear_flags(UCSTPIFG);
        Ok(())
    }

    /// Write `bytes` to `address`.
    pub fn blocking_write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Write(bytes)])
    }

    /// Read from `address` into `buf`.
    pub fn blocking_read(&mut self, address: u8, buf: &mut [u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Read(buf)])
    }

    /// Write `bytes`, then read into `buf` after a repeated start.
    pub fn blocking_write_read(&mut self, address: u8, bytes: &[u8], buf: &mut [u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Write(bytes), Operation::Read(buf)])
    }

    /// Write `bytes` to `address`.
    pub async fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.transaction(address, &mut [Operation::Write(bytes)]).await
    }

    /// Read from `address` into `buf`.
    pub async fn read(&mut self, address: u8, buf: &mut [u8]) -> Result<(), Error> {
        self.transaction(address, &mut [Operation::Read(buf)]).await
    }

    /// Write `bytes`, then read into `buf` after a repeated start.
    pub async fn write_read(&mut self, address: u8, bytes: &[u8], buf: &mut [u8]) -> Result<(), Error> {
        self.transaction(address, &mut [Operation::Write(bytes), Operation::Read(buf)])
            .await
    }
}

impl<'d> Drop for I2c<'d> {
    fn drop(&mut self) {
        self.info.disable_irq(UCRXIFG0 | UCTXIFG0 | UCSTPIFG | FAULTS);
        // Park the module in reset so it releases the bus before the pins go back to GPIO.
        self.info.modify(CTLW0, |v| v | UCSWRST);
        for pin in self.pins.iter() {
            gpio::set_gpio_function(pin);
        }
    }
}

impl embedded_hal::i2c::ErrorType for I2c<'_> {
    type Error = Error;
}

impl embedded_hal::i2c::I2c for I2c<'_> {
    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
        self.blocking_transaction(address, operations)
    }
}

impl embedded_hal_async::i2c::I2c for I2c<'_> {
    async fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
        I2c::transaction(self, address, operations).await
    }
}
