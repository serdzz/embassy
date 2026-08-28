//! What differs between devices.
//!
//! Everything the shared drivers need to know about a particular chip lives behind this module:
//! where its registers are, which of them it has at all, and which peripherals and pins exist. A
//! new device is a new file here plus a feature in `Cargo.toml`.
//!
//! The families this crate covers are not variations on a theme. FR2xx pairs its 8-bit ports into
//! 16-bit halves high in the address map and gives each pin a pull resistor and two function-select
//! bits; F1xx lays its ports out byte by byte down at 0x20, has no pull resistors at all, and one
//! select bit. So the drivers ask questions like "where is this port's direction register" and
//! "does this chip have pulls", rather than assuming an answer.

#[cfg_attr(feature = "msp430fr2355", path = "fr2355.rs")]
#[cfg_attr(feature = "msp430f149", path = "f149.rs")]
#[cfg_attr(feature = "msp430f2618", path = "f2618.rs")]
mod device;

pub(crate) use device::*;

/// A register every port has, whatever the family.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum PortReg {
    /// Pin state.
    In,
    /// Output latch.
    Out,
    /// Direction.
    Dir,
    /// Pull resistor enable. Absent on F1xx.
    Ren,
    /// Interrupt edge select. Only on the ports that can interrupt.
    Ies,
    /// Interrupt enable.
    Ie,
    /// Interrupt flag.
    Ifg,
}

/// Which peripheral a pin is connected to.
///
/// FR2xx encodes this in two bits across `PxSEL0` and `PxSEL1`; F1xx has a single `PxSEL` bit, so
/// only [`PinFunction::Gpio`] and [`PinFunction::Alternate1`] exist there and asking for the others
/// panics.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
// Which of these get used depends on which peripheral drivers the selected device has.
#[allow(dead_code)]
pub(crate) enum PinFunction {
    /// Ordinary input or output.
    Gpio,
    /// The first alternate function. On F1xx, the only one.
    Alternate1,
    /// The second alternate function.
    Alternate2,
    /// The third alternate function, which on FR2xx is the analog one.
    Alternate3,
}
