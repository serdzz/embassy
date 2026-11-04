//! RTC v1 (STM32F1) driver.
//!
//! The F1 RTC is a simple 32-bit up-counter with a 20-bit prescaler and an
//! alarm register, without any calendar functionality. This implementation
//! provides the minimal subset required for low-power wakeup support:
//! prescaler configuration, counter access, and the alarm register.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::pac::rtc::Rtc;
use crate::pac::rtc::vals::Rtoff;
use crate::peripherals::RTC;
use crate::rtc::SealedInstance;

/// Frequency of the RTC counter (after the prescaler), in Hz.
///
/// The prescaler load registers (PRLH/PRLL) are write-only on RTC v1, so the
/// configured counter frequency is cached here for later use by the
/// low-power wakeup alarm code.
pub(crate) static RTC_CNT_HZ: AtomicU32 = AtomicU32::new(0);

#[allow(dead_code)]
impl super::Rtc {
    /// Applies the RTC config: program the prescaler so that the counter
    /// ticks at `cnt_hz`.
    pub(super) fn configure(&mut self, cnt_hz: u32) {
        let rtc_hz = Self::frequency().0;
        let prl = (rtc_hz / cnt_hz).max(1) - 1;
        assert!(prl < (1 << 20), "RTC v1 prescaler must fit in 20 bits");

        RTC_CNT_HZ.store(rtc_hz / (prl + 1), Ordering::Relaxed);

        // Wait for the registers to synchronize after a potential backup
        // domain reset / APB1 reset, so that reads return consistent values.
        let r = RTC::regs();
        r.crl().modify(|w| w.set_rsf(false));
        while !r.crl().read().rsf() {}

        self.write(true, |rtc| {
            rtc.prlh().write(|w| w.set_prlh(((prl >> 16) & 0xf) as u8));
            rtc.prll().write(|w| w.set_prll((prl & 0xffff) as u16));
        });
    }

    /// Read the 32-bit RTC counter.
    pub(crate) fn counter() -> u32 {
        let r = RTC::regs();
        loop {
            let high = r.cnth().read().cnth();
            let low = r.cntl().read().cntl();
            let high_after = r.cnth().read().cnth();

            // A low-to-high carry may occur between the two reads; retry
            // until the high half is stable.
            if high == high_after {
                return ((high as u32) << 16) | low as u32;
            }
        }
    }

    /// Set the 32-bit RTC counter.
    pub(crate) fn set_counter(&mut self, value: u32) {
        self.write(true, |rtc| {
            rtc.cnth().write(|w| w.set_cnth((value >> 16) as u16));
            rtc.cntl().write(|w| w.set_cntl(value as u16));
        });
    }

    /// Perform a write to the RTC registers, entering configuration mode
    /// if requested.
    pub(super) fn write<F, R>(&mut self, init_mode: bool, f: F) -> R
    where
        F: FnOnce(crate::pac::rtc::Rtc) -> R,
    {
        let r = RTC::regs();

        // Wait for any previous write to complete.
        while r.crl().read().rtoff() == Rtoff::ONGOING {}

        if init_mode {
            r.crl().modify(|w| w.set_cnf(true));
        }

        let result = f(r);

        if init_mode {
            r.crl().modify(|w| w.set_cnf(false));
        }

        // Wait for the write to be applied.
        while r.crl().read().rtoff() == Rtoff::ONGOING {}

        result
    }
}

impl SealedInstance for crate::peripherals::RTC {
    // The F1 backup registers live in the separate BKP peripheral, not in
    // the RTC register block, so they are not accessible through this API.
    const BACKUP_REGISTER_COUNT: usize = 0;

    #[cfg(feature = "low-power")]
    const EXTI_WAKEUP_LINE: usize = 17;

    #[cfg(feature = "low-power")]
    type WakeupInterrupt = crate::interrupt::typelevel::RTC_ALARM;

    fn read_backup_register(_rtc: Rtc, _register: usize) -> Option<u32> {
        None
    }

    fn write_backup_register(_rtc: Rtc, _register: usize, _value: u32) {}
}
