//! An [`embassy-time`](embassy_time) driver built on Timer_B0.
//!
//! TB0 runs in continuous mode off ACLK, which is sourced from the internal 32768 Hz REFO
//! oscillator. That gives one tick per `1/32768` s with no prescaler, and it keeps running in LPM3,
//! so the executor can sleep between timer events.
//!
//! The counter is only 16 bits, so it is extended to the 64 bits `embassy-time` wants in software:
//!
//! - `period` counts elapsed half-overflows (2^15 ticks each).
//! - It is bumped both on overflow (`TBIFG`, counter wraps to 0) and half way through
//!   (`CCR1 == 0x8000`).
//!
//! So when `period` is even the counter is in `0..=0x7FFF`, and when it is odd the counter is in
//! `0x8000..=0xFFFF`. That redundancy is what makes [`Driver::now`] safe against racing an
//! overflow: if the counter does not match the parity of the `period` we read, the counter must
//! belong to the next half-period, and [`calc_now`] folds that in.
//!
//! CCR0 is used as the alarm. It has a dedicated interrupt vector (`TIMER0_B0`), separate from the
//! overflow/CCR1 vector (`TIMER0_B1`).

use core::cell::{Cell, RefCell};
use core::task::Waker;

use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;
use msp430fr2355::tb0::tb0iv::Tbiv;
use msp430fr2355::{Cs, Tb0};

const _: () = assert!(
    embassy_time_driver::TICK_HZ == 32_768,
    "this driver ticks at the REFO frequency; enable embassy-time/tick-hz-32_768"
);

/// Half of the 16-bit counter's range. `period` is bumped when the counter reaches this value, and
/// again when it wraps.
const HALF_PERIOD: u16 = 0x8000;

/// How far ahead of `now` an alarm has to be before we stop arming CCR0 and wait for
/// [`Tb0Driver::next_period`] to arm it instead. Anything below one full period is fine; the
/// headroom keeps a nearly-due alarm from being deferred by a whole half-period.
const ARM_HORIZON: u64 = 0xc000;

fn tb0() -> Tb0 {
    // SAFETY: this driver owns TB0. `board::init` hands the peripheral over and never touches it
    // again, and every access below is a single volatile read or write of a register that only
    // this module uses.
    unsafe { Tb0::steal() }
}

/// Combine the software `period` counter with the hardware counter into a 64-bit tick count.
///
/// `period` is a `u32` of half-periods, so this overflows after `2^32 * 2^15 / 32768` seconds,
/// i.e. about 136 years of uptime.
fn calc_now(period: u32, counter: u16) -> u64 {
    ((period as u64) << 15) + ((counter as u32 ^ ((period & 1) << 15)) as u64)
}

struct Tb0Driver {
    /// Number of 2^15 tick periods elapsed since [`init`].
    period: Mutex<Cell<u32>>,
    /// Tick at which CCR0 is set to fire, or `u64::MAX` when the alarm is disarmed.
    alarm_at: Mutex<Cell<u64>>,
    queue: Mutex<RefCell<Queue>>,
}

embassy_time_driver::time_driver_impl!(static DRIVER: Tb0Driver = Tb0Driver {
    period: Mutex::new(Cell::new(0)),
    alarm_at: Mutex::new(Cell::new(u64::MAX)),
    queue: Mutex::new(RefCell::new(Queue::new())),
});

impl Tb0Driver {
    /// Account for a half-period that just elapsed, and arm the alarm if it now falls inside the
    /// window CCR0 can express.
    fn next_period(&self) {
        critical_section::with(|cs| {
            // `period` is only ever written here, from the timer interrupt, so this can't race.
            let period = self.period.borrow(cs).get().wrapping_add(1);
            self.period.borrow(cs).set(period);

            let t = (period as u64) << 15;
            if self.alarm_at.borrow(cs).get() < t + ARM_HORIZON {
                // `set_alarm` already wrote the right compare value, just let it through.
                tb0().tb0cctl0().modify(|_, w| w.ccifg().ccifg_0().ccie().ccie_1());
            }
        })
    }

    /// Wake everything that is due and re-arm the alarm for whatever comes next.
    fn trigger_alarm(&self, cs: CriticalSection) {
        let mut next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
        while !self.set_alarm(cs, next) {
            next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
        }
    }

    /// Arm CCR0 for `timestamp`.
    ///
    /// Returns `false` if `timestamp` is already in the past, meaning the alarm will *not* fire and
    /// the caller has to handle the expiry itself.
    fn set_alarm(&self, cs: CriticalSection, timestamp: u64) -> bool {
        let t = tb0();
        self.alarm_at.borrow(cs).set(timestamp);

        if timestamp <= self.now() {
            t.tb0cctl0().modify(|_, w| w.ccie().ccie_0());
            self.alarm_at.borrow(cs).set(u64::MAX);
            return false;
        }

        // Write the compare value even when we leave the interrupt masked: `next_period` unmasks it
        // later without recomputing anything.
        t.tb0ccr0().write(|w| unsafe { w.bits(timestamp as u16) });

        // Only unmask if the alarm falls within the current 16-bit window. Otherwise CCR0 would
        // match on some earlier wrap of the counter.
        if timestamp - self.now() < ARM_HORIZON {
            t.tb0cctl0().modify(|_, w| w.ccifg().ccifg_0().ccie().ccie_1());
        } else {
            t.tb0cctl0().modify(|_, w| w.ccie().ccie_0());
        }

        // The counter may have passed `timestamp` while we were setting all this up, in which case
        // we can't tell whether CCR0 matched or not. Disarm and report it as missed.
        if timestamp <= self.now() {
            t.tb0cctl0().modify(|_, w| w.ccie().ccie_0());
            self.alarm_at.borrow(cs).set(u64::MAX);
            return false;
        }

        true
    }
}

impl Driver for Tb0Driver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| {
            let period = self.period.borrow(cs).get();
            let counter = tb0().tb0r().read().bits();
            calc_now(period, counter)
        })
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();

            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        })
    }
}

/// Start the clock. Called once by [`crate::board::init`].
pub(crate) fn init() {
    critical_section::with(|_| {
        // SAFETY: called once, before anything else configures the clock system.
        let cs = unsafe { Cs::steal() };
        // Drive ACLK from REFO rather than XT1: no crystal needed, and it survives LPM3.
        cs.csctl4().modify(|_, w| w.sela().refoclk());

        let t = tb0();

        // Continuous mode, ACLK, no divider, counter cleared, overflow interrupt on.
        t.tb0ctl().write(|w| {
            w.tbssel()
                .aclk()
                .id()
                ._1()
                .mc()
                .continuous()
                .tbclr()
                .set_bit()
                .tbie()
                .tbie_1()
        });

        // CCR1 gives us the half-overflow tick that keeps `period`'s parity meaningful.
        t.tb0ccr1().write(|w| unsafe { w.bits(HALF_PERIOD) });
        t.tb0cctl1().write(|w| w.ccie().ccie_1());

        // CCR0 is the alarm. It stays masked until `set_alarm` has something to wait for.
        t.tb0cctl0().write(|w| w.ccie().ccie_0());
    })
}

embassy_executor::msp430_interrupt! {
    /// CCR0 compare match: an `embassy-time` alarm came due.
    unsafe fn TIMER0_B0() {
        // CCR0 has its own vector and no TB0IV entry, so its flag has to be cleared by hand.
        tb0().tb0cctl0().modify(|_, w| w.ccifg().ccifg_0().ccie().ccie_0());
        critical_section::with(|cs| DRIVER.trigger_alarm(cs));
    }

    /// Counter overflow and CCR1 half-overflow: keeps the 64-bit tick count moving.
    unsafe fn TIMER0_B1() {
        // Reading TB0IV clears the highest-priority flag it reports.
        match tb0().tb0iv().read().tbiv().variant() {
            Some(Tbiv::Tbccr1) | Some(Tbiv::Tbifg) => DRIVER.next_period(),
            _ => {}
        }
    }
}
