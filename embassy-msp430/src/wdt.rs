//! Watchdog timer.
//!
//! The watchdog runs out of reset, so [`crate::init`] stops it. Call [`Watchdog::start`] to put it
//! back to work.
//!
//! WDTCTL is password protected: every write has to carry `0x5A` in its high byte, and a write
//! without it resets the chip. That is why this module never does a read-modify-write on it and
//! keeps the whole register value in software instead.

use crate::pac;

/// Password that has to accompany every write to `WDTCTL`.
const WDTPW: u16 = 0x5A00;
const WDTHOLD: u16 = 0x0080;
const WDTCNTCL: u16 = 0x0008;

/// Clock the watchdog counts.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtClock {
    /// SMCLK. Stops in LPM2 and deeper, so the watchdog stops with it.
    #[default]
    Smclk,
    /// ACLK. Keeps counting down to LPM3.
    Aclk,
    /// VLOCLK, about 10 kHz. Keeps counting in every low-power mode below LPM4.
    Vlo,
}

impl WdtClock {
    const fn bits(self) -> u16 {
        match self {
            WdtClock::Smclk => 0,
            WdtClock::Aclk => 1,
            WdtClock::Vlo => 2,
        }
    }
}

/// Watchdog interval, as a divider of the selected clock.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(non_camel_case_types)]
pub enum WdtInterval {
    /// 2^31 clocks.
    _2G,
    /// 2^27 clocks.
    _128M,
    /// 2^23 clocks.
    _8M,
    /// 2^19 clocks.
    _512K,
    /// 2^15 clocks.
    _32K,
    /// 2^13 clocks.
    _8K,
    /// 2^9 clocks.
    _512,
    /// 2^6 clocks.
    _64,
}

impl WdtInterval {
    const fn bits(self) -> u16 {
        self as u16
    }

    /// Number of clocks in this interval.
    pub const fn clocks(self) -> u32 {
        match self {
            WdtInterval::_2G => 1 << 31,
            WdtInterval::_128M => 1 << 27,
            WdtInterval::_8M => 1 << 23,
            WdtInterval::_512K => 1 << 19,
            WdtInterval::_32K => 1 << 15,
            WdtInterval::_8K => 1 << 13,
            WdtInterval::_512 => 1 << 9,
            WdtInterval::_64 => 1 << 6,
        }
    }
}

#[inline]
fn write_ctl(value: u16) {
    // SAFETY: single volatile write of a fully-formed register value, password included.
    unsafe { pac::WdtA::steal() }
        .wdtctl()
        .write(|w| unsafe { w.bits(WDTPW | value) });
}

/// Stop the watchdog.
///
/// [`crate::init`] does this for you.
pub fn stop() {
    write_ctl(WDTHOLD);
}

/// The watchdog timer, in watchdog mode.
///
/// If [`Watchdog::feed`] is not called within the configured interval, the chip resets.
pub struct Watchdog {
    ctl: u16,
}

impl Watchdog {
    /// Start the watchdog on `clock` with the given `interval`.
    pub fn start(clock: WdtClock, interval: WdtInterval) -> Self {
        // WDTTMSEL stays 0, which is watchdog mode rather than interval-timer mode.
        let ctl = (clock.bits() << 5) | interval.bits();
        // Clearing the counter as it starts means the first interval is a full one.
        write_ctl(ctl | WDTCNTCL);
        Self { ctl }
    }

    /// Reset the counter, postponing the reset by another full interval.
    pub fn feed(&mut self) {
        write_ctl(self.ctl | WDTCNTCL);
    }

    /// Stop the watchdog.
    pub fn stop(self) {
        write_ctl(self.ctl | WDTHOLD);
    }
}
