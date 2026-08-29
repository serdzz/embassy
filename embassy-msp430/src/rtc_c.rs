//! RTC_C: a calendar clock that keeps date and time while everything else is asleep.
//!
//! Not the same peripheral as the FR2xx [`crate::rtc`], which is a counter that this crate turns
//! into an interval timer. This one holds seconds, minutes, hours, day, month and year, keeps them
//! correct across month lengths and leap years in hardware, and carries on doing it in LPM3.5 with
//! the CPU unpowered — which is the whole point on a meter that has to timestamp what it recorded.
//!
//! # BCD, not binary
//!
//! The counters are configured in BCD, because the alarm registers compare in BCD whatever the
//! counters do, and having the two disagree is a bug waiting to be written. Conversion happens at
//! the edge of this module, so nothing outside it sees a packed nibble.
//!
//! # Reading a clock that is running
//!
//! Reading second, minute and hour separately can straddle a tick and produce a time that never
//! existed — 01:59:60 read as 01:00:00. The hardware answers this with `RTCRDY`, which is low while
//! the counters are changing. [`Rtc::now`] waits for it and re-reads if it drops mid-read.

use embassy_hal_internal::Peri;
use embassy_sync::waitqueue::AtomicWaker;

use crate::peripherals;

const BASE: u16 = 0x04a0;

const CTL0: u16 = 0x00;
const CTL13: u16 = 0x02;
const TIM0: u16 = 0x10;
const TIM1: u16 = 0x12;
const DATE: u16 = 0x14;
const YEAR: u16 = 0x16;
const AMINHR: u16 = 0x18;
const ADOWDAY: u16 = 0x1a;

// RTCCTL0. The high byte is a key: writes without it are ignored and set an access-violation flag.
const RTCKEY: u16 = 0xa500;
const RTCAIFG: u16 = 1 << 1;
const RTCTEVIFG: u16 = 1 << 2;
const RTCAIE: u16 = 1 << 5;
const RTCTEVIE: u16 = 1 << 6;

// RTCCTL13.
const RTCRDY: u16 = 1 << 4;
const RTCHOLD: u16 = 1 << 6;
const RTCBCD: u16 = 1 << 7;

/// The alarm's enable bit, which lives in the top bit of each alarm register.
const ALARM_ENABLE: u8 = 0x80;

static WAKER: AtomicWaker = AtomicWaker::new();

/// Clock the calendar counts from.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    /// The watch crystal on LFXT, through the prescalers. What a real clock uses.
    #[default]
    Xt1,
    /// The internal low-frequency oscillator. Keeps counting with no crystal fitted, and drifts by
    /// minutes a day.
    Vlo,
}

impl ClockSource {
    /// `RTCSSEL` value.
    const fn bits(self) -> u16 {
        match self {
            ClockSource::Xt1 => 0,
            ClockSource::Vlo => 1,
        }
    }
}

/// A date and time, as people write them.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DateTime {
    /// Full year, 1900 to 4095.
    pub year: u16,
    /// 1 to 12.
    pub month: u8,
    /// 1 to 31.
    pub day: u8,
    /// 0 is Sunday.
    pub weekday: u8,
    /// 0 to 23.
    pub hour: u8,
    /// 0 to 59.
    pub minute: u8,
    /// 0 to 59.
    pub second: u8,
}

/// Why a date was refused.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// A field was outside what a calendar allows.
    ///
    /// The hardware would take it and then count on from nonsense, so it is refused here instead.
    InvalidDateTime,
}

/// How often the periodic event fires.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interval {
    /// Every minute, as the seconds roll over.
    #[default]
    Minute,
    /// Every hour.
    Hour,
    /// Every midnight.
    Midnight,
    /// Every noon.
    Noon,
}

impl Interval {
    /// `RTCTEV` value.
    const fn bits(self) -> u16 {
        match self {
            Interval::Minute => 0,
            Interval::Hour => 1,
            Interval::Midnight => 2,
            Interval::Noon => 3,
        }
    }
}

#[inline]
fn read(offset: u16) -> u16 {
    // SAFETY: a volatile read of an RTC_C register.
    unsafe { ((BASE + offset) as *mut u16).read_volatile() }
}

#[inline]
fn write(offset: u16, value: u16) {
    // SAFETY: a volatile write to an RTC_C register.
    unsafe { ((BASE + offset) as *mut u16).write_volatile(value) }
}

/// Write `RTCCTL0`, which needs the key in its high byte.
#[inline]
fn write_ctl0(value: u16) {
    write(CTL0, RTCKEY | (value & 0xff));
}

/// Read `RTCCTL0`'s low byte.
#[inline]
fn read_ctl0() -> u16 {
    read(CTL0) & 0xff
}

const fn to_bcd(value: u8) -> u8 {
    ((value / 10) << 4) | (value % 10)
}

const fn from_bcd(value: u8) -> u8 {
    ((value >> 4) * 10) + (value & 0x0f)
}

/// Days in `month` of `year`, so a date can be refused before the hardware counts on from it.
const fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 0,
    }
}

impl DateTime {
    /// Is this a date that exists?
    const fn is_valid(&self) -> bool {
        self.year >= 1900
            && self.year <= 4095
            && self.month >= 1
            && self.month <= 12
            && self.day >= 1
            && self.day <= days_in_month(self.year, self.month)
            && self.weekday <= 6
            && self.hour <= 23
            && self.minute <= 59
            && self.second <= 59
    }
}

/// The calendar clock.
pub struct Rtc<'d> {
    _peri: Peri<'d, peripherals::RTC>,
}

impl<'d> Rtc<'d> {
    /// Start the clock at `now`.
    ///
    /// The counters are held while they are written, so the clock cannot tick between the seconds
    /// being set and the year being set.
    pub fn new(
        peri: Peri<'d, peripherals::RTC>,
        source: ClockSource,
        now: DateTime,
    ) -> Result<Self, Error> {
        let mut rtc = Self { _peri: peri };

        write(CTL13, RTCHOLD | RTCBCD | (source.bits() << 2));
        write_ctl0(0);
        rtc.set(now)?;

        Ok(rtc)
    }

    /// Set the clock.
    pub fn set(&mut self, now: DateTime) -> Result<(), Error> {
        if !now.is_valid() {
            return Err(Error::InvalidDateTime);
        }

        let ctl13 = read(CTL13);
        write(CTL13, ctl13 | RTCHOLD);

        write(TIM0, ((to_bcd(now.minute) as u16) << 8) | to_bcd(now.second) as u16);
        write(TIM1, ((now.weekday as u16) << 8) | to_bcd(now.hour) as u16);
        write(DATE, ((to_bcd(now.month) as u16) << 8) | to_bcd(now.day) as u16);
        write(
            YEAR,
            ((to_bcd((now.year / 100) as u8) as u16) << 8) | to_bcd((now.year % 100) as u8) as u16,
        );

        write(CTL13, ctl13 & !RTCHOLD);
        Ok(())
    }

    /// The current date and time.
    ///
    /// Waits for the counters to be still. A read that started while they were changing is thrown
    /// away and taken again rather than returned, because the alternative is a time that never
    /// happened.
    pub fn now(&self) -> DateTime {
        loop {
            while read(CTL13) & RTCRDY == 0 {}

            let tim0 = read(TIM0);
            let tim1 = read(TIM1);
            let date = read(DATE);
            let year = read(YEAR);

            // Still ready means nothing rolled over underneath the four reads.
            if read(CTL13) & RTCRDY == 0 {
                continue;
            }

            return DateTime {
                second: from_bcd(tim0 as u8),
                minute: from_bcd((tim0 >> 8) as u8),
                hour: from_bcd(tim1 as u8),
                weekday: (tim1 >> 8) as u8 & 0x07,
                day: from_bcd(date as u8),
                month: from_bcd((date >> 8) as u8),
                year: from_bcd((year >> 8) as u8) as u16 * 100 + from_bcd(year as u8) as u16,
            };
        }
    }

    /// Fire the alarm at the next time matching every field given.
    ///
    /// A `None` field is a wildcard, which is how the hardware expresses "every hour at twenty
    /// past": pass `minute: Some(20)` and leave the rest `None`. All four `None` disables the
    /// alarm.
    pub fn set_alarm(
        &mut self,
        minute: Option<u8>,
        hour: Option<u8>,
        day_of_week: Option<u8>,
        day_of_month: Option<u8>,
    ) {
        // Every alarm register has to be disabled before any is changed, or a half-written alarm
        // can match on the way through.
        write(AMINHR, 0);
        write(ADOWDAY, 0);

        let field = |v: Option<u8>, bcd: bool| -> u16 {
            match v {
                Some(v) => (ALARM_ENABLE | if bcd { to_bcd(v) } else { v }) as u16,
                None => 0,
            }
        };

        write(AMINHR, field(minute, true) | (field(hour, true) << 8));
        write(
            ADOWDAY,
            field(day_of_week, false) | (field(day_of_month, true) << 8),
        );
    }

    /// Ask for the periodic event, and clear anything stale.
    fn arm_interval(&mut self, interval: Interval) {
        let ctl13 = read(CTL13) & !0x0003;
        write(CTL13, ctl13 | interval.bits());
        write_ctl0(read_ctl0() & !RTCTEVIFG);
    }

    /// Wait for the next `interval` boundary.
    pub async fn wait_interval(&mut self, interval: Interval) {
        self.arm_interval(interval);
        wait_for(RTCTEVIFG, RTCTEVIE).await;
    }

    /// Wait for the alarm set by [`Rtc::set_alarm`].
    pub async fn wait_alarm(&mut self) {
        write_ctl0(read_ctl0() & !RTCAIFG);
        wait_for(RTCAIFG, RTCAIE).await;
    }
}

/// Wait until `flag` is up, with `enable` unmasked while waiting.
///
/// The handler masks rather than clears, so the flag is still there to be found here; clearing it
/// is this function's job, once the waiting is over.
async fn wait_for(flag: u16, enable: u16) {
    core::future::poll_fn(|cx| {
        if read_ctl0() & flag != 0 {
            return core::task::Poll::Ready(());
        }
        WAKER.register(cx.waker());
        if read_ctl0() & flag != 0 {
            return core::task::Poll::Ready(());
        }
        write_ctl0(read_ctl0() | enable);
        core::task::Poll::Pending
    })
    .await;

    write_ctl0(read_ctl0() & !(flag | enable));
}

impl Drop for Rtc<'_> {
    fn drop(&mut self) {
        write(CTL13, read(CTL13) | RTCHOLD);
        write_ctl0(0);
    }
}

embassy_executor::msp430_interrupt! {
    /// The alarm or the periodic event.
    ///
    /// Which one is left for the waiting task to work out from the flags; the handler only masks
    /// the enables so it cannot re-enter, and wakes whoever is waiting.
    unsafe fn RTC_C() {
        // The four enables sit exactly four bits above their flags, so shifting the enables down
        // and masking against the flags leaves a bit set for each source that is both asserted and
        // wanted. Getting this wrong would be a hang rather than a missed event: the handler would
        // return without masking, and an interrupt whose flag stays up fires again immediately.
        let ctl0 = read_ctl0();
        let active = (ctl0 >> 4) & ctl0 & 0x0f;
        if active == 0 {
            return;
        }
        write_ctl0(ctl0 & !(RTCAIE | RTCTEVIE));
        WAKER.wake();
    }
}
