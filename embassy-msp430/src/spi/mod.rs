//! Serial peripheral interface, master mode.
//!
//! The synchronous peripheral differs by family — the FRAM parts have the eUSCI, the F1xx parts the
//! older USART — but they are asked the same questions, so [`Config`] and [`Error`] are shared and
//! only the driver behind them changes.
//!
//! Three-wire master only: chip select is left to you as an ordinary
//! [`Output`](crate::gpio::Output), which is what [`embedded_hal::spi::SpiDevice`] implementations
//! expect anyway.

#[cfg_attr(feature = "msp430fr2355", path = "eusci.rs")]
#[cfg_attr(feature = "_fr504x_604x", path = "eusci.rs")]
#[cfg_attr(feature = "msp430fr4133", path = "eusci.rs")]
#[cfg_attr(feature = "msp430f149", path = "usart.rs")]
mod device;

pub use device::*;
pub use embedded_hal::spi::{MODE_0, MODE_1, MODE_2, MODE_3, Mode, Phase, Polarity};

use crate::clock::PeripheralClock;

/// Order the bits of a byte go out in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BitOrder {
    /// Most significant bit first. What almost every device expects.
    #[default]
    MsbFirst,
    /// Least significant bit first.
    LsbFirst,
}

/// SPI configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Config {
    /// Bit clock in Hz. Rounded down to what the integer divider can produce.
    pub frequency: u32,
    /// Clock polarity and phase.
    pub mode: Mode,
    /// Bit order.
    pub bit_order: BitOrder,
    /// Clock the bit rate generator runs from.
    pub clock_source: PeripheralClock,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 1_000_000,
            mode: MODE_0,
            bit_order: BitOrder::default(),
            clock_source: PeripheralClock::default(),
        }
    }
}

/// Reasons a [`Config`] cannot be applied.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// The divider cannot reach this bit clock from the selected source.
    UnachievableFrequency,
    /// [`crate::init`] has not run, so the clock frequencies are unknown.
    ClocksNotInitialized,
    /// This device's shift register only goes one way round. The F1xx USART is most significant
    /// bit first and has no control over it.
    UnsupportedBitOrder,
}

/// SPI errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// A character arrived before the previous one had been read.
    Overrun,
}

impl embedded_hal::spi::Error for Error {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        match self {
            Error::Overrun => embedded_hal::spi::ErrorKind::Overrun,
        }
    }
}
