//! The FR4xx clock system: an FLL-trimmed DCO for MCLK and SMCLK, and a choice of low-frequency
//! source for ACLK.
//!
//! The same CS module as the FR2xx, two notches smaller: the DCO tops out at 16 MHz instead of 24,
//! ACLK can only come from XT1 or REFO (no VLO option in `SELA`, which is a single bit here), and
//! the SMCLK divider stops at /8.
//!
//! Out of reset the FLL runs the DCO at about 1 MHz off the internal REFO oscillator, and ACLK is
//! taken from XT1.

use core::arch::asm;

#[allow(unused_imports)]
use PeripheralClock as _;

use super::{Clocks, PeripheralClock};
use crate::pac;

/// Source for ACLK, the low-frequency clock that keeps running in LPM3.
///
/// The [`embassy-time`](https://docs.rs/embassy-time) driver is clocked from ACLK, so this also
/// decides how accurate timekeeping is and how deep the executor may sleep.
///
/// Unlike the FR2xx CS, there is no VLO option: `SELA` is one bit on this family.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AclkSource {
    /// Internal 32768 Hz reference oscillator. Always available, no external parts, ±3.5%.
    #[default]
    Refo,
    /// External low-frequency crystal on XT1. Accurate, but the board has to have one fitted.
    ///
    /// The clock system falls back to REFO on its own if XT1 fails to start.
    Xt1,
}

/// DCO frequency, which MCLK and SMCLK are derived from.
///
/// The FLL locks the DCO to ACLK's reference oscillator, so these are the frequencies the FLL can
/// hit exactly from a 32768 Hz reference. The FR4xx DCO stops at 16 MHz.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcoFreq {
    /// About 1 MHz. The reset default, and the only setting that needs no FRAM wait states.
    #[default]
    _1MHz,
    /// 7.96 MHz.
    _8MHz,
    /// 15.97 MHz. Adds one FRAM wait state.
    _16MHz,
}

impl DcoFreq {
    /// `(DCORSEL, FLLN, NWAITS)`.
    ///
    /// `FLLN + 1` is the ratio between the DCO and the 32768 Hz FLL reference, so the frequency
    /// really produced is `(FLLN + 1) * 32768`.
    const fn params(self) -> (u16, u16, u8) {
        match self {
            // DCORSEL_0, the reset value; the FLL is left alone.
            DcoFreq::_1MHz => (0, 30, 0),
            DcoFreq::_8MHz => (3, 243, 0),
            DcoFreq::_16MHz => (5, 487, 1),
        }
    }

    /// Resulting DCO frequency in Hz.
    pub const fn hz(self) -> u32 {
        (self.params().1 as u32 + 1) * 32_768
    }
}

/// Divider applied to the DCO to produce MCLK.
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
    /// Divide by 64.
    _64,
    /// Divide by 128.
    _128,
}

impl Div {
    const fn bits(self) -> u8 {
        self as u8
    }

    const fn factor(self) -> u32 {
        1 << (self as u32)
    }
}

/// Divider between MCLK and SMCLK. On this family SMCLK is derived from MCLK, not from the DCO
/// directly, and the divider field is two bits.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmclkDiv {
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

impl SmclkDiv {
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
    /// DCO frequency, before the MCLK/SMCLK dividers.
    pub dco: DcoFreq,
    /// MCLK divider. MCLK clocks the CPU.
    pub mclk_div: Div,
    /// SMCLK divider, applied on top of the MCLK divider. SMCLK clocks most peripherals.
    pub smclk_div: SmclkDiv,
    /// ACLK source.
    pub aclk: AclkSource,
}

/// SCG0 bit of the status register. Setting it holds the FLL while it is being reprogrammed.
const SCG0: u16 = 0x0040;

/// Password for the FRAM controller registers.
const FRCTLPW: u16 = 0xA500;

/// Apply `config`.
///
/// # Safety
///
/// Changing MCLK underneath running peripherals will change their timing. Call this before
/// anything else is configured, which is what [`crate::init`] does.
pub(crate) unsafe fn init(config: Config) -> Clocks {
    let cs = unsafe { pac::Cs::steal() };
    let (dcorsel, flln, nwaits) = config.dco.params();

    // FRAM cannot keep up with MCLK above 8 MHz without wait states. Raise them before speeding
    // the DCO up, and lower them only after slowing it down.
    if nwaits > 0 {
        set_fram_waits(nwaits);
    }

    if config.dco != DcoFreq::_1MHz {
        // Hold the FLL while its dividers are inconsistent, otherwise it chases a bogus target.
        unsafe { asm!("bis #{scg0}, r2", scg0 = const SCG0, options(nostack)) };

        // Reference the FLL to REFO: it is always there, unlike XT1.
        cs.csctl3().modify(|_, w| unsafe { w.bits(0x0010) }); // SELREF__REFOCLK, REFDIV = /1
        cs.csctl0().write(|w| unsafe { w.bits(0) }); // clear the DCO tap, the FLL will re-find it
        cs.csctl1()
            .modify(|r, w| unsafe { w.bits((r.bits() & !0x000E) | (dcorsel << 1)) });
        cs.csctl2().write(|w| unsafe { w.bits(flln) }); // FLLD = /1

        // The FLL needs a few cycles with the new settings before it may be released.
        for _ in 0..3 {
            msp430::asm::nop();
        }

        unsafe { asm!("bic #{scg0}, r2", scg0 = const SCG0, options(nostack)) };

        // FLLUNLOCK is two bits; both clear means locked.
        while cs.csctl7().read().bits() & 0x0003 != 0 {}
    }

    // SELMS selects the source for MCLK (and through it SMCLK); DCOCLKDIV is 0. SELA is a single
    // bit here: 0 is XT1, 1 is REFO.
    let sela = match config.aclk {
        AclkSource::Xt1 => 0u16,
        AclkSource::Refo => 1,
    };
    cs.csctl4()
        .modify(|r, w| unsafe { w.bits((r.bits() & !0x0107) | (sela << 8)) });
    cs.csctl5().modify(|r, w| unsafe {
        w.bits((r.bits() & !0x0037) | ((config.smclk_div.bits() as u16) << 4) | config.mclk_div.bits() as u16)
    });

    if nwaits == 0 {
        set_fram_waits(0);
    }

    let dco = config.dco.hz();
    let mclk = dco / config.mclk_div.factor();
    Clocks {
        mclk,
        smclk: mclk / config.smclk_div.factor(),
        aclk: 32_768,
    }
}

fn set_fram_waits(nwaits: u8) {
    let frctl = unsafe { pac::Fram::steal() };
    frctl
        .frctl0()
        .write(|w| unsafe { w.bits(FRCTLPW | ((nwaits as u16) << 4)) });
}
