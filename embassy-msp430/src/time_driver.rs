//! [`embassy-time`](embassy_time_driver) driver built on Timer_B0.
//!
//! TB0 runs in continuous mode off ACLK with no prescaler, so one tick is `1/32768` s and the
//! counter wraps every two seconds. ACLK keeps running in LPM3, which is what lets the executor
//! sleep between timer events.
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
//! CCR0 is the alarm. It has a dedicated interrupt vector, separate from the overflow/CCR1 one.

use core::cell::{Cell, RefCell};
use core::task::Waker;

use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

use crate::chip;

/// Half of the 16-bit counter's range. `period` is bumped when the counter reaches this value, and
/// again when it wraps.
const HALF_PERIOD: u16 = 0x8000;

/// How far ahead of `now` an alarm has to be before we stop arming CCR0 and let
/// [`Tb0Driver::next_period`] arm it instead. Anything below one full period works; the headroom
/// keeps a nearly-due alarm from being deferred by a whole half-period.
const ARM_HORIZON: u64 = 0xc000;

// Timer_B register offsets from the control register, which is where `chip::TB0_BASE` points.
const CTL: u16 = 0x00;
const CCTL0: u16 = 0x02;
const CCTL1: u16 = 0x04;
const R: u16 = 0x10;
const CCR0: u16 = 0x12;
const CCR1: u16 = 0x14;

// Control register bits.
const TBIE: u16 = 0x0002;
const TBCLR: u16 = 0x0004;
const MC_CONTINUOUS: u16 = 0x0020;
const TBSSEL_ACLK: u16 = 0x0100;

// Capture/compare control bits.
const CCIFG: u16 = 0x0001;
const CCIE: u16 = 0x0010;

#[inline]
fn read(offset: u16) -> u16 {
    // SAFETY: this driver owns the timer. `crate::init` never hands it out, and this is a plain
    // volatile read of one of its registers.
    unsafe { ((chip::TB0_BASE + offset) as *mut u16).read_volatile() }
}

#[inline]
fn write(offset: u16, value: u16) {
    // SAFETY: as above, a volatile write to a register only this module touches.
    unsafe { ((chip::TB0_BASE + offset) as *mut u16).write_volatile(value) }
}

#[inline]
fn modify(offset: u16, f: impl FnOnce(u16) -> u16) {
    critical_section::with(|_| write(offset, f(read(offset))));
}

/// Combine the software `period` counter with the hardware counter into a 64-bit tick count.
///
/// `period` counts half-periods in a `u32`, so this overflows after `2^32 * 2^15 / 32768` seconds,
/// about 136 years of uptime.
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
                modify(CCTL0, |v| (v & !CCIFG) | CCIE);
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
        self.alarm_at.borrow(cs).set(timestamp);

        if timestamp <= self.now() {
            modify(CCTL0, |v| v & !CCIE);
            self.alarm_at.borrow(cs).set(u64::MAX);
            return false;
        }

        // Write the compare value even when the interrupt stays masked: `next_period` unmasks it
        // later without recomputing anything.
        write(CCR0, timestamp as u16);

        // Only unmask if the alarm falls within the current 16-bit window. Otherwise CCR0 would
        // match on an earlier wrap of the counter.
        if timestamp - self.now() < ARM_HORIZON {
            modify(CCTL0, |v| (v & !CCIFG) | CCIE);
        } else {
            modify(CCTL0, |v| v & !CCIE);
        }

        // The counter may have passed `timestamp` while we were setting all this up, in which case
        // we can't tell whether CCR0 matched. Disarm and report it as missed.
        if timestamp <= self.now() {
            modify(CCTL0, |v| v & !CCIE);
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
            let counter = read(R);
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

/// Start the clock. Called once by [`crate::init`].
pub(crate) fn init(_cs: CriticalSection) {
    // Continuous mode, ACLK, no divider, counter cleared, overflow interrupt on.
    write(CTL, TBSSEL_ACLK | MC_CONTINUOUS | TBCLR | TBIE);

    // CCR1 gives us the half-overflow tick that keeps `period`'s parity meaningful.
    write(CCR1, HALF_PERIOD);
    write(CCTL1, CCIE);

    // CCR0 is the alarm. It stays masked until `set_alarm` has something to wait for.
    write(CCTL0, 0);
}

#[cfg(any(feature = "msp430fr2355", feature = "_fr504x_604x"))]
embassy_executor::msp430_interrupt! {
    /// CCR0 compare match: an `embassy-time` alarm came due.
    unsafe fn TIMER0_B0() {
        // CCR0 has its own vector and no entry in the vector register, so its flag has to be
        // cleared by hand.
        modify(CCTL0, |v| v & !(CCIFG | CCIE));
        critical_section::with(|cs| DRIVER.trigger_alarm(cs));
    }

    /// Counter overflow and CCR1 half-overflow: keeps the 64-bit tick count moving.
    unsafe fn TIMER0_B1() {
        // Reading the vector register clears the highest-priority flag it reports. 2 is CCR1, 14
        // is the counter overflow.
        // SAFETY: a volatile read of this timer's vector register.
        match unsafe { (chip::TB0_IV as *mut u16).read_volatile() } {
            2 | 14 => DRIVER.next_period(),
            _ => {}
        }
    }
}

// The FR4133 has no Timer_B, so the driver runs on Timer0_A3 — same register layout, same control
// bits, same TAxIV encodings; only the names of the vectors differ.
#[cfg(feature = "msp430fr4133")]
embassy_executor::msp430_interrupt! {
    /// CCR0 compare match: an `embassy-time` alarm came due.
    unsafe fn TIMER0_A0() {
        // CCR0 has its own vector and no entry in the vector register, so its flag has to be
        // cleared by hand.
        modify(CCTL0, |v| v & !(CCIFG | CCIE));
        critical_section::with(|cs| DRIVER.trigger_alarm(cs));
    }

    /// Counter overflow and CCR1 half-overflow: keeps the 64-bit tick count moving.
    unsafe fn TIMER0_A1() {
        // Reading the vector register clears the highest-priority flag it reports. 2 is CCR1, 14
        // is the counter overflow.
        // SAFETY: a volatile read of this timer's vector register.
        match unsafe { (chip::TB0_IV as *mut u16).read_volatile() } {
            2 | 14 => DRIVER.next_period(),
            _ => {}
        }
    }
}

#[cfg(any(feature = "msp430f149", feature = "msp430f2618"))]
embassy_executor::msp430_interrupt! {
    /// CCR0 compare match: an `embassy-time` alarm came due.
    unsafe fn TIMERB0() {
        // CCR0 has its own vector and no entry in the vector register, so its flag has to be
        // cleared by hand.
        modify(CCTL0, |v| v & !(CCIFG | CCIE));
        critical_section::with(|cs| DRIVER.trigger_alarm(cs));
    }

    /// Counter overflow and CCR1 half-overflow: keeps the 64-bit tick count moving.
    unsafe fn TIMERB1() {
        // Reading the vector register clears the highest-priority flag it reports. 2 is CCR1, 14
        // is the counter overflow.
        // SAFETY: a volatile read of this timer's vector register.
        match unsafe { (chip::TB0_IV as *mut u16).read_volatile() } {
            2 | 14 => DRIVER.next_period(),
            _ => {}
        }
    }
}
