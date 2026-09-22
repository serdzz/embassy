//! Real-time clock counter.
//!
//! A 16-bit counter with its own prescaler that counts up to [`Rtc::set_period`] and starts again,
//! raising an interrupt each time it wraps.
//!
//! The [`embassy-time`](embassy_time) driver already gives you timers, so the reason to reach for
//! this is what it can be clocked from: [`RtcClock::Vlo`] is a separate, very low power oscillator,
//! so a slow periodic wakeup can run off it while ACLK, the timer and the DCO all stay off. It also
//! frees Timer_B0 if you would rather spend it on something else.
//!
//! VLO is cheap rather than accurate — tens of percent of spread over temperature and voltage — so
//! use it for waking up, not for keeping time.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::Peri;
use embassy_sync::waitqueue::AtomicWaker;

use crate::peripherals;

#[cfg(feature = "msp430fr2355")]
const BASE: u16 = 0x0300;
/// The FR4133's counter has the same registers at a different address.
#[cfg(feature = "msp430fr4133")]
const BASE: u16 = 0x03c0;

// Control register.
const RTCIFG: u16 = 0x0001;
const RTCIE: u16 = 0x0002;
const RTCSR: u16 = 0x0040;
const RTCPS_SHIFT: u16 = 8;
const RTCSS_SHIFT: u16 = 12;

// Register offsets from the peripheral base.
const CTL: u16 = 0x00;
const IV: u16 = 0x04;
const MOD: u16 = 0x08;
const CNT: u16 = 0x0c;

static WAKER: AtomicWaker = AtomicWaker::new();

#[inline]
fn read(offset: u16) -> u16 {
    // SAFETY: a volatile read of an RTC register.
    unsafe { ((BASE + offset) as *mut u16).read_volatile() }
}

#[inline]
fn write(offset: u16, value: u16) {
    // SAFETY: a volatile write to an RTC register.
    unsafe { ((BASE + offset) as *mut u16).write_volatile(value) }
}

#[inline]
fn modify(offset: u16, f: impl FnOnce(u16) -> u16) {
    critical_section::with(|_| write(offset, f(read(offset))));
}

/// Clock the counter counts.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcClock {
    /// The very-low-power oscillator, nominally 10 kHz. Runs in every low-power mode down to
    /// LPM4, and costs almost nothing, but is only accurate to tens of percent.
    #[default]
    Vlo,
    /// The low-frequency crystal on XT1, 32768 Hz. Accurate, if the board has one fitted.
    Xt1,
    /// SMCLK. Fast, but it stops in LPM2 and deeper, which defeats the point.
    Smclk,
}

impl RtcClock {
    const fn bits(self) -> u16 {
        match self {
            RtcClock::Smclk => 1,
            RtcClock::Xt1 => 2,
            RtcClock::Vlo => 3,
        }
    }
}

/// Divider between the source clock and the counter.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescaler {
    /// Divide by 1.
    _1,
    /// Divide by 10.
    #[default]
    _10,
    /// Divide by 100.
    _100,
    /// Divide by 1000.
    _1000,
    /// Divide by 16.
    _16,
    /// Divide by 64.
    _64,
}

impl Prescaler {
    const fn bits(self) -> u16 {
        self as u16
    }

    /// What the divider actually divides by.
    pub const fn divisor(self) -> u32 {
        match self {
            Prescaler::_1 => 1,
            Prescaler::_10 => 10,
            Prescaler::_100 => 100,
            Prescaler::_1000 => 1000,
            Prescaler::_16 => 16,
            Prescaler::_64 => 64,
        }
    }
}

/// RTC configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// Clock the counter counts.
    pub clock: RtcClock,
    /// Divider between that clock and the counter.
    pub prescaler: Prescaler,
}

/// Nominal VLO frequency. It is not trimmed, so treat this as an order of magnitude.
const VLO_HZ: u32 = 10_000;
/// XT1 is a watch crystal wherever it is fitted.
const XT1_HZ: u32 = 32_768;

/// The real-time clock counter.
pub struct Rtc<'d> {
    _peri: Peri<'d, peripherals::RTC>,
    tick_hz: u32,
}

impl<'d> Rtc<'d> {
    /// Start the counter.
    ///
    /// It free-runs from zero with the largest possible period until [`Rtc::set_period`] says
    /// otherwise.
    pub fn new(peri: Peri<'d, peripherals::RTC>, config: Config) -> Self {
        let source_hz = match config.clock {
            RtcClock::Vlo => VLO_HZ,
            RtcClock::Xt1 => XT1_HZ,
            // Falls back to zero if `init` has not run, which only makes `tick_hz` meaningless;
            // the counter itself still works.
            RtcClock::Smclk => crate::clocks().map_or(0, |c| c.smclk),
        };

        write(CTL, 0);
        write(MOD, u16::MAX);
        write(
            CTL,
            (config.clock.bits() << RTCSS_SHIFT) | (config.prescaler.bits() << RTCPS_SHIFT) | RTCSR,
        );

        Self {
            _peri: peri,
            tick_hz: source_hz / config.prescaler.divisor(),
        }
    }

    /// How many times a second the counter advances.
    ///
    /// Zero if the source is SMCLK and [`crate::init`] has not run.
    pub fn tick_hz(&self) -> u32 {
        self.tick_hz
    }

    /// Wrap, and fire, every `ticks` counts.
    ///
    /// The counter restarts from zero, so the first period is a whole one.
    pub fn set_period(&mut self, ticks: u16) {
        write(MOD, ticks.saturating_sub(1));
        modify(CTL, |v| v | RTCSR);
    }

    /// Wrap, and fire, every `millis` milliseconds, as closely as the prescaler allows.
    ///
    /// Returns `false` if the period does not fit the 16-bit counter at the configured tick rate,
    /// leaving the previous period in place.
    pub fn set_period_millis(&mut self, millis: u32) -> bool {
        let ticks = (self.tick_hz as u64 * millis as u64) / 1000;
        if ticks == 0 || ticks > u16::MAX as u64 {
            return false;
        }
        self.set_period(ticks as u16);
        true
    }

    /// Current counter value.
    pub fn count(&self) -> u16 {
        read(CNT)
    }

    /// Wait for the counter to wrap.
    ///
    /// A wrap that happened before this was called counts: the flag is sticky, so nothing is
    /// missed between two calls, but only one is remembered.
    pub async fn wait(&mut self) {
        poll_fn(|cx| {
            critical_section::with(|_| {
                if read(CTL) & RTCIFG != 0 {
                    // Reading the vector register is what clears the flag.
                    let _ = read(IV);
                    Poll::Ready(())
                } else {
                    // Register before unmasking: the handler cannot run until this critical
                    // section ends, so the wakeup cannot be missed.
                    WAKER.register(cx.waker());
                    modify(CTL, |v| v | RTCIE);
                    Poll::Pending
                }
            })
        })
        .await
    }

    /// Forget any wrap that has already happened, so the next [`Rtc::wait`] waits for a fresh one.
    pub fn clear_pending(&mut self) {
        let _ = read(IV);
    }
}

impl<'d> Drop for Rtc<'d> {
    fn drop(&mut self) {
        // Selecting no source stops the counter, which is the whole of what it costs to run.
        write(CTL, 0);
    }
}

embassy_executor::msp430_interrupt! {
    /// Counter wrapped.
    unsafe fn RTC() {
        // The flag is cleared by reading RTCIV, which is the task's job: doing it here would lose
        // the very event being reported. Masking instead both silences the interrupt and leaves
        // the flag as the "has it wrapped" answer.
        modify(CTL, |v| v & !RTCIE);
        WAKER.wake();
    }
}
