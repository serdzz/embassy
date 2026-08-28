//! The F1xx basic clock system (BCS+): a free-running DCO for MCLK and SMCLK, and the LFXT1
//! crystal for ACLK.
//!
//! There is no FLL here and no factory calibration, so **the DCO's frequency is not something the
//! chip knows**. [`Config::dco_hz`] is you telling the HAL what it actually runs at, so that
//! peripheral drivers can work out their dividers; changing [`Config::rsel`] without updating it
//! will silently skew every baud rate derived from SMCLK.
//!
//! ACLK is the LFXT1 crystal, and F1xx has no internal low-frequency oscillator to fall back on. A
//! board without its watch crystal has no ACLK at all, which stops the
//! [`embassy-time`](https://docs.rs/embassy-time) driver dead. A watch crystal also takes up to a
//! second to start, so time runs slow for a moment after reset.

use super::{Clocks, PeripheralClock};

const DCOCTL: u16 = 0x0056;
const BCSCTL1: u16 = 0x0057;
const BCSCTL2: u16 = 0x0058;

/// LFXT1 with a watch crystal, which is the only thing ACLK can be on this family.
const ACLK_HZ: u32 = 32_768;

/// Divider applied to a clock.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Div {
    /// Divide by 1.
    #[default]
    _1,
    /// Divide by 2.
    _2,
    /// Divide by 4.
    _4,
    /// Divide by 8.
    _8,
}

impl Div {
    const fn bits(self) -> u8 {
        self as u8
    }

    const fn factor(self) -> u32 {
        1 << (self as u32)
    }
}

/// Clock system configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// DCO frequency range, 0 to 7. Higher is faster, roughly a factor of 1.6 per step.
    ///
    /// The reset value is 4, which lands somewhere around 800 kHz.
    pub rsel: u8,
    /// What the DCO actually runs at, in Hz.
    ///
    /// Nothing measures this: it is what [`Clocks::mclk`] and [`Clocks::smclk`] are computed from,
    /// and it is only as right as the number you put here. The default matches the reset state.
    pub dco_hz: u32,
    /// MCLK divider. MCLK clocks the CPU.
    pub mclk_div: Div,
    /// SMCLK divider. SMCLK clocks most peripherals.
    pub smclk_div: Div,
    /// ACLK divider.
    pub aclk_div: Div,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rsel: 4,
            dco_hz: 800_000,
            mclk_div: Div::default(),
            smclk_div: Div::default(),
            aclk_div: Div::default(),
        }
    }
}

#[inline]
fn write(addr: u16, value: u8) {
    // SAFETY: a volatile write to a clock system register.
    unsafe { (addr as *mut u8).write_volatile(value) }
}

#[inline]
fn read(addr: u16) -> u8 {
    // SAFETY: a volatile read of a clock system register.
    unsafe { (addr as *mut u8).read_volatile() }
}

/// Apply `config`.
///
/// # Safety
///
/// Changing MCLK underneath running peripherals will change their timing. Call this before anything
/// else is configured, which is what [`crate::init`] does.
pub(crate) unsafe fn init(config: Config) -> Clocks {
    assert!(config.rsel <= 7, "RSEL is a three-bit field");

    // BCSCTL1: keep XT2OFF and XTS as the reset values (XT2 off, LFXT1 in low-frequency mode, which
    // is what a watch crystal needs), set the ACLK divider and the DCO range.
    let bcsctl1 = (read(BCSCTL1) & !0x3F) | (config.aclk_div.bits() << 4) | config.rsel;
    write(BCSCTL1, bcsctl1);

    // DCOCTL: middle tap of the range, modulation off. The FLL would normally be dialling these in;
    // with none, the middle is the least surprising place to sit.
    write(DCOCTL, 0x60);

    // BCSCTL2: MCLK and SMCLK both from the DCO, with their dividers. DCOR stays clear, so the
    // internal resistor sets the DCO current.
    write(BCSCTL2, (config.mclk_div.bits() << 4) | (config.smclk_div.bits() << 1));

    Clocks {
        mclk: config.dco_hz / config.mclk_div.factor(),
        smclk: config.dco_hz / config.smclk_div.factor(),
        aclk: ACLK_HZ / config.aclk_div.factor(),
    }
}

#[allow(unused_imports)]
use PeripheralClock as _;
