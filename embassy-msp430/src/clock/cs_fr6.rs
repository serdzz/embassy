//! The FR5xx/FR6xx clock system: a DCO with a fixed set of calibrated frequencies.
//!
//! Not to be confused with the FR2xx module of the same name in [`super`]. They share a family
//! resemblance and nothing else: the FR2xx one has an FLL that multiplies a reference up to
//! whatever is asked for, while this one has a DCO trimmed at the factory to one of a dozen
//! frequencies and no FLL at all. Different registers, at a different address, with different
//! fields.
//!
//! # FRAM wait states are part of the clock
//!
//! FRAM cannot be read at full speed above 8 MHz, so the FRAM controller inserts wait states, and
//! how many depends on MCLK. Getting that wrong does not fail cleanly — the CPU fetches garbage.
//! So [`init`] sets the wait states itself, from the frequency it is about to configure, rather
//! than leaving it to be forgotten somewhere else.

use super::{Clocks, PeripheralClock};

const CSCTL0: u16 = 0x0160;
const CSCTL1: u16 = 0x0162;
const CSCTL2: u16 = 0x0164;
const CSCTL3: u16 = 0x0166;
const CSCTL4: u16 = 0x0168;

/// Password for `CSCTL0`. Writing the register without it triggers a reset, which is the point.
const CSKEY: u16 = 0xa500;

/// FRAM controller, whose wait states go with MCLK.
const FRCTL0: u16 = 0x0140;
/// Password for `FRCTL0`, same idea as `CSKEY`.
const FRCTLPW: u16 = 0xa500;

/// Nominal VLO frequency. Untrimmed, so treat it as an order of magnitude.
const VLO_HZ: u32 = 10_000;
/// A watch crystal on LFXT, wherever one is fitted.
const LFXT_HZ: u32 = 32_768;

/// DCO frequency.
///
/// These are the frequencies the factory trimmed; nothing in between is reachable. The awkward ones
/// are real — 2.67 and 5.33 MHz are 8 MHz divided by three and by one and a half.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcoFreq {
    /// 1 MHz.
    #[default]
    _1MHz,
    /// 2.67 MHz.
    _2_67MHz,
    /// 3.5 MHz.
    _3_5MHz,
    /// 4 MHz.
    _4MHz,
    /// 5.33 MHz.
    _5_33MHz,
    /// 7 MHz.
    _7MHz,
    /// 8 MHz.
    _8MHz,
    /// 16 MHz. Needs one FRAM wait state.
    _16MHz,
    /// 21 MHz. Needs two FRAM wait states.
    _21MHz,
    /// 24 MHz. Needs two FRAM wait states.
    _24MHz,
}

impl DcoFreq {
    /// `(DCORSEL, DCOFSEL)`, the two fields of `CSCTL1` that pick the frequency.
    ///
    /// The same `DCOFSEL` means different things on either side of `DCORSEL`, which is why this is
    /// a table rather than arithmetic.
    const fn bits(self) -> u16 {
        const DCORSEL: u16 = 1 << 4;
        match self {
            DcoFreq::_1MHz => 0,
            DcoFreq::_2_67MHz => 1 << 1,
            DcoFreq::_3_5MHz => 2 << 1,
            DcoFreq::_4MHz => 3 << 1,
            DcoFreq::_5_33MHz => 4 << 1,
            DcoFreq::_7MHz => 5 << 1,
            DcoFreq::_8MHz => 6 << 1,
            DcoFreq::_16MHz => DCORSEL | (4 << 1),
            DcoFreq::_21MHz => DCORSEL | (5 << 1),
            DcoFreq::_24MHz => DCORSEL | (6 << 1),
        }
    }

    /// Resulting frequency in Hz.
    ///
    /// The thirds are rounded to the nearest hertz, which is far finer than the DCO's own tolerance.
    pub const fn hz(self) -> u32 {
        match self {
            DcoFreq::_1MHz => 1_000_000,
            DcoFreq::_2_67MHz => 2_666_667,
            DcoFreq::_3_5MHz => 3_500_000,
            DcoFreq::_4MHz => 4_000_000,
            DcoFreq::_5_33MHz => 5_333_333,
            DcoFreq::_7MHz => 7_000_000,
            DcoFreq::_8MHz => 8_000_000,
            DcoFreq::_16MHz => 16_000_000,
            DcoFreq::_21MHz => 21_000_000,
            DcoFreq::_24MHz => 24_000_000,
        }
    }
}

/// Source for ACLK, the low-frequency clock that keeps running in the deeper low-power modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AclkSource {
    /// The 32768 Hz crystal on LFXT. Accurate, if the board has one.
    #[default]
    Lfxt,
    /// The internal very-low-power oscillator, about 10 kHz. No crystal needed, accurate to tens of
    /// percent.
    Vlo,
}

impl AclkSource {
    /// `SELA` field value.
    const fn bits(self) -> u16 {
        match self {
            AclkSource::Lfxt => 0,
            AclkSource::Vlo => 1,
        }
    }

    const fn hz(self) -> u32 {
        match self {
            AclkSource::Lfxt => LFXT_HZ,
            AclkSource::Vlo => VLO_HZ,
        }
    }
}

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
    /// Divide by 16.
    _16,
    /// Divide by 32.
    _32,
}

impl Div {
    const fn bits(self) -> u16 {
        self as u16
    }

    const fn factor(self) -> u32 {
        1 << (self as u32)
    }
}

/// Clock system configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// DCO frequency, before the MCLK and SMCLK dividers.
    pub dco: DcoFreq,
    /// MCLK divider. MCLK clocks the CPU.
    pub mclk_div: Div,
    /// SMCLK divider. SMCLK clocks most peripherals.
    pub smclk_div: Div,
    /// ACLK source.
    pub aclk: AclkSource,
    /// ACLK divider.
    pub aclk_div: Div,
}

#[inline]
fn write(addr: u16, value: u16) {
    // SAFETY: a volatile write to a clock system or FRAM controller register.
    unsafe { (addr as *mut u16).write_volatile(value) }
}

#[inline]
fn read(addr: u16) -> u16 {
    // SAFETY: a volatile read of a clock system register.
    unsafe { (addr as *mut u16).read_volatile() }
}

/// Wait states the FRAM needs at `mclk`.
///
/// One up to 16 MHz, two up to 24. Rounding up is free — a wait state too many costs a cycle, a
/// wait state too few costs correctness.
const fn wait_states(mclk: u32) -> u16 {
    if mclk > 16_000_000 {
        2
    } else if mclk > 8_000_000 {
        1
    } else {
        0
    }
}

/// Apply `config`.
///
/// # Safety
///
/// Changing MCLK underneath running peripherals changes their timing. Call this before anything
/// else is configured, which is what [`crate::init`] does.
pub(crate) unsafe fn init(config: Config) -> Clocks {
    let dco_hz = config.dco.hz();
    let mclk = dco_hz / config.mclk_div.factor();

    // Wait states first. Going the other way — raising MCLK before the FRAM is ready for it — is
    // the ordering that fetches garbage.
    write(FRCTL0, FRCTLPW | (wait_states(mclk) << 4));

    // CSCTL0 unlocks the rest of the block; it stays unlocked until something writes it wrongly.
    write(CSCTL0, CSKEY);

    // Frequency before sources, so that nothing runs briefly at the reset default.
    write(CSCTL1, config.dco.bits());

    // ACLK from its chosen source; MCLK and SMCLK from the DCO. SELM and SELS take 3 for DCOCLK.
    write(CSCTL2, (config.aclk.bits() << 8) | (3 << 4) | 3);

    write(
        CSCTL3,
        (config.aclk_div.bits() << 8) | (config.smclk_div.bits() << 4) | config.mclk_div.bits(),
    );

    // Turn off what is not in use. HFXT is off either way — this device's high-frequency crystal
    // input is not something the HAL drives yet — and LFXT stays on only if ACLK needs it.
    const LFXTOFF: u16 = 1 << 0;
    const VLOOFF: u16 = 1 << 3;
    const HFXTOFF: u16 = 1 << 8;
    let mut ctl4 = read(CSCTL4) | HFXTOFF;
    match config.aclk {
        AclkSource::Lfxt => {
            ctl4 &= !LFXTOFF;
            ctl4 |= VLOOFF;
        }
        AclkSource::Vlo => {
            ctl4 |= LFXTOFF;
            ctl4 &= !VLOOFF;
        }
    }
    write(CSCTL4, ctl4);

    Clocks {
        mclk,
        smclk: dco_hz / config.smclk_div.factor(),
        aclk: config.aclk.hz() / config.aclk_div.factor(),
    }
}

#[allow(unused_imports)]
use PeripheralClock as _;
