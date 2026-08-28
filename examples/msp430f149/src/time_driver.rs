//! An [`embassy-time`](embassy_time_driver) driver on the F149's Timer_B7.
//!
//! The same scheme as the FR2355 one, and for the same reasons — TB runs in continuous mode off
//! ACLK with no prescaler, so a tick is `1/32768` s and the counter wraps every two seconds, and
//! the 16-bit counter is extended to 64 bits by counting half-overflows in software:
//!
//! - `period` counts elapsed half-overflows (2^15 ticks each).
//! - It is bumped both on overflow (`TBIFG`) and half way through (`CCR1 == 0x8000`).
//!
//! So `period`'s parity says which half of the counter's range we are in, which is what makes
//! [`Driver::now`] safe against racing an overflow: a counter that disagrees with the parity must
//! belong to the next half-period, and [`calc_now`] folds that in. CCR0 is the alarm.
//!
//! What differs from the FR2xx parts is where the registers live. `TBIV` sits at 0x011E, well away
//! from the timer block at 0x0180, and ACLK has no internal source to fall back on — see
//! [`crate::board`].

use core::cell::{Cell, RefCell};
use core::task::Waker;

use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

use crate::pac;

/// Half of the 16-bit counter's range.
const HALF_PERIOD: u16 = 0x8000;

/// How far ahead of `now` an alarm has to be before we let [`Tb7Driver::next_period`] arm it
/// instead of arming CCR0 now.
const ARM_HORIZON: u64 = 0xc000;

fn tb() -> pac::TimerB7 {
    // SAFETY: this driver owns TB7. `board::init` hands the peripheral over and never touches it
    // again, and every access below is a single volatile read or write.
    unsafe { pac::TimerB7::steal() }
}

/// Combine the software `period` counter with the hardware counter into a 64-bit tick count.
fn calc_now(period: u32, counter: u16) -> u64 {
    ((period as u64) << 15) + ((counter as u32 ^ ((period & 1) << 15)) as u64)
}

struct Tb7Driver {
    period: Mutex<Cell<u32>>,
    alarm_at: Mutex<Cell<u64>>,
    queue: Mutex<RefCell<Queue>>,
}

embassy_time_driver::time_driver_impl!(static DRIVER: Tb7Driver = Tb7Driver {
    period: Mutex::new(Cell::new(0)),
    alarm_at: Mutex::new(Cell::new(u64::MAX)),
    queue: Mutex::new(RefCell::new(Queue::new())),
});

impl Tb7Driver {
    fn next_period(&self) {
        critical_section::with(|cs| {
            // `period` is only ever written here, from the timer interrupt, so this can't race.
            let period = self.period.borrow(cs).get().wrapping_add(1);
            self.period.borrow(cs).set(period);

            let t = (period as u64) << 15;
            if self.alarm_at.borrow(cs).get() < t + ARM_HORIZON {
                // `set_alarm` already wrote the right compare value, just let it through.
                tb().tbcctl0().modify(|_, w| w.ccifg().clear_bit().ccie().set_bit());
            }
        })
    }

    fn trigger_alarm(&self, cs: CriticalSection) {
        let mut next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
        while !self.set_alarm(cs, next) {
            next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
        }
    }

    /// Arm CCR0 for `timestamp`, or report that it is already in the past.
    fn set_alarm(&self, cs: CriticalSection, timestamp: u64) -> bool {
        let t = tb();
        self.alarm_at.borrow(cs).set(timestamp);

        if timestamp <= self.now() {
            t.tbcctl0().modify(|_, w| w.ccie().clear_bit());
            self.alarm_at.borrow(cs).set(u64::MAX);
            return false;
        }

        // Write the compare value even while the interrupt stays masked: `next_period` unmasks it
        // later without recomputing anything.
        t.tbccr0().write(|w| unsafe { w.bits(timestamp as u16) });

        // Only unmask if the alarm falls within the current 16-bit window; otherwise CCR0 would
        // match on an earlier wrap of the counter.
        if timestamp - self.now() < ARM_HORIZON {
            t.tbcctl0().modify(|_, w| w.ccifg().clear_bit().ccie().set_bit());
        } else {
            t.tbcctl0().modify(|_, w| w.ccie().clear_bit());
        }

        // The counter may have passed `timestamp` while this was being set up, in which case we
        // cannot tell whether CCR0 matched. Disarm and report it as missed.
        if timestamp <= self.now() {
            t.tbcctl0().modify(|_, w| w.ccie().clear_bit());
            self.alarm_at.borrow(cs).set(u64::MAX);
            return false;
        }

        true
    }
}

impl Driver for Tb7Driver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| {
            let period = self.period.borrow(cs).get();
            let counter = tb().tbr().read().bits();
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
pub(crate) fn init(timer: pac::TimerB7) {
    // Continuous mode (MC = 2), ACLK (TBSSEL = 1), no divider, counter cleared, overflow on.
    timer.tbctl().write(|w| {
        w.tbssel()
            .tbssel_1()
            .id()
            .id_0()
            .mc()
            .mc_2()
            .tbclr()
            .set_bit()
            .tbie()
            .set_bit()
    });

    // CCR1 gives the half-overflow tick that keeps `period`'s parity meaningful.
    timer.tbccr1().write(|w| unsafe { w.bits(HALF_PERIOD) });
    timer.tbcctl1().write(|w| w.ccie().set_bit());

    // CCR0 is the alarm. It stays masked until `set_alarm` has something to wait for.
    timer.tbcctl0().write(|w| w.ccie().clear_bit());
}

embassy_executor::msp430_interrupt! {
    /// CCR0 compare match: an `embassy-time` alarm came due.
    unsafe fn TIMERB0() {
        // CCR0 has its own vector and no TBIV entry, so its flag has to be cleared by hand.
        tb().tbcctl0().modify(|_, w| w.ccifg().clear_bit().ccie().clear_bit());
        critical_section::with(|cs| DRIVER.trigger_alarm(cs));
    }

    /// Counter overflow and CCR1 half-overflow: keeps the 64-bit tick count moving.
    unsafe fn TIMERB1() {
        // Reading TBIV clears the highest-priority flag it reports. 2 is CCR1, 14 is the overflow.
        match tb().tbiv().read().bits() {
            2 | 14 => DRIVER.next_period(),
            _ => {}
        }
    }
}
