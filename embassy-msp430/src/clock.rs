//! Clock system (CS) configuration.
//!
//! Out of reset the FLL runs the DCO at about 1 MHz off the internal REFO oscillator, and ACLK is
//! taken from XT1. This module lets you pick a different DCO frequency and a different ACLK source
//! without touching the FLL by hand.

use core::arch::asm;

use crate::pac;

/// Source for ACLK, the low-frequency clock that keeps running in LPM3.
///
/// The [`embassy-time`](https://docs.rs/embassy-time) driver is clocked from ACLK, so this also
/// decides how accurate timekeeping is and how deep the executor may sleep.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AclkSource {
    /// Internal 32768 Hz reference oscillator. Always available, no external parts, ±3.5%.
    #[default]
    Refo,
    /// Internal very-low-power oscillator, about 10 kHz. Cheapest to run, but coarse.
    Vlo,
    /// External low-frequency crystal on XT1. Accurate, but the board has to have one fitted.
    ///
    /// The clock system falls back to REFO on its own if XT1 fails to start.
    Xt1,
}

/// DCO frequency, which MCLK and SMCLK are derived from.
///
/// The FLL locks the DCO to ACLK's reference oscillator, so these are the frequencies the FLL can
/// hit exactly from a 32768 Hz reference.
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
    /// 23.95 MHz. Adds two FRAM wait states.
    _24MHz,
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
            DcoFreq::_24MHz => (7, 731, 2),
        }
    }

    /// Resulting DCO frequency in Hz.
    pub const fn hz(self) -> u32 {
        (self.params().1 as u32 + 1) * 32_768
    }
}

/// Divider applied to the DCO to produce MCLK or SMCLK.
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

/// Clock system configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// DCO frequency, before the MCLK/SMCLK dividers.
    pub dco: DcoFreq,
    /// MCLK divider. MCLK clocks the CPU.
    pub mclk_div: Div,
    /// SMCLK divider. SMCLK clocks most peripherals.
    pub smclk_div: Div,
    /// ACLK source.
    pub aclk: AclkSource,
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

    // SELMS selects the source for both MCLK and SMCLK; DCOCLKDIV is 0.
    let sela = match config.aclk {
        AclkSource::Xt1 => 0u16,
        AclkSource::Refo => 1,
        AclkSource::Vlo => 2,
    };
    cs.csctl4()
        .modify(|r, w| unsafe { w.bits((r.bits() & !0x0303) | (sela << 8)) });
    cs.csctl5().modify(|r, w| unsafe {
        w.bits((r.bits() & !0x0077) | ((config.smclk_div.bits() as u16) << 4) | config.mclk_div.bits() as u16)
    });

    if nwaits == 0 {
        set_fram_waits(0);
    }

    let dco = config.dco.hz();
    Clocks {
        mclk: dco / config.mclk_div.factor(),
        smclk: dco / config.smclk_div.factor(),
        aclk: match config.aclk {
            AclkSource::Vlo => 10_000,
            _ => 32_768,
        },
    }
}

fn set_fram_waits(nwaits: u8) {
    let frctl = unsafe { pac::Frctl::steal() };
    frctl
        .frctl0()
        .write(|w| unsafe { w.bits(FRCTLPW | ((nwaits as u16) << 4)) });
}
