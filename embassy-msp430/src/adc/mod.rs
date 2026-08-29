//! Analog-to-digital converter.
//!
//! Unlike the serial peripherals, the two families' converters do not answer the same questions:
//! the FR2xx one has selectable resolution and three internal references, the F1xx ADC12 is twelve
//! bits always and has two. So this module has no shared vocabulary — the whole API comes from the
//! device behind it, and porting code between families means revisiting the configuration.

#[cfg_attr(feature = "msp430fr2355", path = "fr2xx.rs")]
#[cfg_attr(feature = "msp430f149", path = "adc12.rs")]
#[cfg_attr(feature = "msp430fr6043", path = "adc12b.rs")]
mod device;

pub use device::*;
