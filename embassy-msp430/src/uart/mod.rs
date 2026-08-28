//! Universal asynchronous receiver/transmitter.
//!
//! The serial peripheral differs by family — the FRAM parts have the eUSCI_A, the F1xx parts the
//! older USART — but they are asked the same questions, so [`Config`] and [`Error`] are shared and
//! only the driver behind them changes.

#[cfg_attr(feature = "msp430fr2355", path = "eusci.rs")]
#[cfg_attr(feature = "msp430f149", path = "usart.rs")]
mod device;

pub use device::*;

use crate::clock::PeripheralClock;

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
