//! The F2xx basic clock system: a factory-trimmed DCO for MCLK and SMCLK, and a choice of
//! low-frequency source for ACLK.
//!
//! This is the same BCS+ as the F1xx one in every respect but the two that matter. The DCO has
//! **calibration constants burned into information memory** at the factory, so asking for 8 MHz
//! gives 8 MHz to within a percent rather than the guess an F1xx forces; and ACLK can come from the
//! internal VLO, so a board with no watch crystal still has a low-frequency clock and the
//! [`embassy-time`](https://docs.rs/embassy-time) driver still runs.
//!
//! The calibration constants are in segment A of information memory, which is erasable. A chip
//! whose segment A has been erased reads 0xFF for every constant; [`init`] refuses to use those
//! rather than run the DCO somewhere unpredictable.

use super::{Clocks, PeripheralClock};

const DCOCTL: u16 = 0x0056;
const BCSCTL1: u16 = 0x0057;
const BCSCTL2: u16 = 0x0058;
const BCSCTL3: u16 = 0x0053;

/// Factory DCO calibration, in information memory segment A.
const CALDCO_16MHZ: u16 = 0x10f8;
const CALBC1_16MHZ: u16 = 0x10f9;
const CALDCO_12MHZ: u16 = 0x10fa;
const CALBC1_12MHZ: u16 = 0x10fb;
const CALDCO_8MHZ: u16 = 0x10fc;
const CALBC1_8MHZ: u16 = 0x10fd;
const CALDCO_1MHZ: u16 = 0x10fe;
const CALBC1_1MHZ: u16 = 0x10ff;

/// Nominal VLO frequency. It is not trimmed, so treat this as an order of magnitude.
const VLO_HZ: u32 = 12_000;
/// A watch crystal, wherever one is fitted.
const XT1_HZ: u32 = 32_768;

/// Source for ACLK, the low-frequency clock that keeps running in LPM3.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AclkSource {
    /// The low-frequency crystal on XT1, 32768 Hz. Accurate, if the board has one fitted.
    #[default]
    Xt1,
    /// The internal very-low-power oscillator, about 12 kHz. No crystal needed, and it costs
    /// almost nothing to run, but it is accurate only to tens of percent.
    Vlo,
}

impl AclkSource {
    /// `LFXT1S` field.
    const fn bits(self) -> u8 {
        match self {
            AclkSource::Xt1 => 0,
            AclkSource::Vlo => 2,
        }
    }

    const fn hz(self) -> u32 {
        match self {
            AclkSource::Xt1 => XT1_HZ,
            AclkSource::Vlo => VLO_HZ,
        }
    }
}

/// DCO frequency, chosen from the ones the factory trimmed.
///
/// Others are reachable by setting `RSEL` and `DCOCTL` by hand, but then nothing knows what the
/// result is, and every baud rate derived from SMCLK becomes a guess. These four are exact.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcoFreq {
    /// 1 MHz.
    #[default]
    _1MHz,
    /// 8 MHz.
    _8MHz,
    /// 12 MHz.
    _12MHz,
    /// 16 MHz. Needs a supply of at least 3.3 V.
    _16MHz,
}

impl DcoFreq {
    /// `(CALDCO address, CALBC1 address)`.
    const fn calibration(self) -> (u16, u16) {
        match self {
            DcoFreq::_1MHz => (CALDCO_1MHZ, CALBC1_1MHZ),
            DcoFreq::_8MHz => (CALDCO_8MHZ, CALBC1_8MHZ),
            DcoFreq::_12MHz => (CALDCO_12MHZ, CALBC1_12MHZ),
            DcoFreq::_16MHz => (CALDCO_16MHZ, CALBC1_16MHZ),
        }
    }

    /// Resulting frequency in Hz.
    pub const fn hz(self) -> u32 {
        match self {
            DcoFreq::_1MHz => 1_000_000,
            DcoFreq::_8MHz => 8_000_000,
            DcoFreq::_12MHz => 12_000_000,
            DcoFreq::_16MHz => 16_000_000,
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
fn read(addr: u16) -> u8 {
    // SAFETY: a volatile read of a clock system register or of information memory.
    unsafe { (addr as *mut u8).read_volatile() }
}

#[inline]
fn write(addr: u16, value: u8) {
    // SAFETY: a volatile write to a clock system register.
    unsafe { (addr as *mut u8).write_volatile(value) }
}

/// Apply `config`.
///
/// Panics if the DCO calibration constants have been erased, since running the DCO at an unknown
/// frequency would silently break every baud rate and timeout derived from SMCLK.
///
/// # Safety
///
/// Changing MCLK underneath running peripherals will change their timing. Call this before anything
/// else is configured, which is what [`crate::init`] does.
pub(crate) unsafe fn init(config: Config) -> Clocks {
    let (caldco, calbc1) = config.dco.calibration();
    let dco = read(caldco);
    let bc1 = read(calbc1);
    assert!(
        dco != 0xff && bc1 != 0xff,
        "the DCO calibration constants have been erased from information memory"
    );

    // The calibration pair carries RSEL in BCSCTL1 and the tap and modulation in DCOCTL, so both
    // have to be written, DCOCTL last.
    //
    // XT2 stays off and XTS stays clear, which is LFXT1 in low-frequency mode — what both a watch
    // crystal and the VLO need. The ACLK divider lives in the same register.
    write(BCSCTL1, (bc1 & 0x0f) | (config.aclk_div.bits() << 4) | 0x80);
    write(DCOCTL, dco);

    // LFXT1S picks the crystal or the VLO. XCAP stays at its reset value, which suits the usual
    // 12.5 pF watch crystal.
    write(BCSCTL3, (read(BCSCTL3) & !0x30) | (config.aclk.bits() << 4));

    // MCLK and SMCLK both from the DCO, with their dividers.
    write(BCSCTL2, (config.mclk_div.bits() << 4) | (config.smclk_div.bits() << 1));

    let dco_hz = config.dco.hz();
    Clocks {
        mclk: dco_hz / config.mclk_div.factor(),
        smclk: dco_hz / config.smclk_div.factor(),
        aclk: config.aclk.hz() / config.aclk_div.factor(),
    }
}

#[allow(unused_imports)]
use PeripheralClock as _;
