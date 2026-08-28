//! Clock system configuration.
//!
//! The families differ completely here — FR2xx has the CS module with an FLL, F1xx has BCS+ with a
//! free-running DCO — so [`Config`] is whatever the selected device's clock system takes. What they
//! agree on is the answer: [`Clocks`], the frequencies everything else derives its dividers from.

#[cfg_attr(feature = "msp430fr2355", path = "cs.rs")]
#[cfg_attr(feature = "msp430f149", path = "bcs.rs")]
mod device;

pub use device::*;

/// Clock a peripheral's bit rate generator runs from.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeripheralClock {
    /// SMCLK. Fast enough for high bit rates, but it stops in LPM2 and deeper.
    #[default]
    Smclk,
    /// ACLK, 32768 Hz. Slow, but the peripheral keeps running down to LPM3.
    Aclk,
}

/// Frequencies the clock system was actually configured for.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Clocks {
    /// MCLK, the CPU clock, in Hz.
    pub mclk: u32,
    /// SMCLK, the peripheral clock, in Hz.
    pub smclk: u32,
    /// ACLK, in Hz.
    pub aclk: u32,
}
