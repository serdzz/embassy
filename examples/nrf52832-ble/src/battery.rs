//! Battery voltage and charge level of the Feather's LiPo.
//!
//! VBAT reaches A7 (P0.31, AIN7) through a resistor divider. The SAADC samples it against its
//! internal 0.6 V reference with a gain of 1/6, a 0..3.6 V range.

use core::sync::atomic::{AtomicU8, AtomicU16, Ordering};

use embassy_nrf::interrupt::{InterruptExt, Priority};
use embassy_nrf::saadc::{self, ChannelConfig, Oversample, Saadc, Time};
use embassy_nrf::{Peri, bind_interrupts, interrupt, peripherals};
use embassy_time::Timer;

bind_interrupts!(struct Irqs {
    SAADC => saadc::InterruptHandler;
});

/// VBAT / voltage at A7 (×1000). The divider on the nRF52832 Feather is 806 kΩ over 2 MΩ, the
/// value Adafruit's own VBAT example uses for this board.
const DIVIDER_X1000: u32 = 1403;

/// The last measurement; 0 until the first one.
pub static MILLIVOLTS: AtomicU16 = AtomicU16::new(0);
pub static PERCENT: AtomicU8 = AtomicU8::new(0);

/// Open-circuit LiPo voltage (mV) against remaining charge (%), at rest and room temperature.
const CURVE: [(u16, u8); 11] = [
    (3300, 0),
    (3610, 5),
    (3690, 10),
    (3730, 20),
    (3770, 30),
    (3800, 40),
    (3840, 50),
    (3870, 60),
    (3950, 70),
    (4020, 80),
    (4100, 90),
];
/// Observed open-circuit voltage right after the ETA6096 ends CV charging
/// and cuts off: the pack relaxes from its ~4.2 V regulation point down to
/// here, so this (not 4200) is what "fully charged" actually reads as on
/// this board/battery (measured: 4137 mV at 93% before this fix).
const FULL_MV: u16 = 4140;

/// How far the voltage has to move past a step before the reported level follows it.
const HYSTERESIS_MV: u16 = 10;

/// Remaining charge for a battery voltage, interpolated between the points of [`CURVE`].
pub fn percent(mv: u16) -> u8 {
    let mut prev = (CURVE[0].0, CURVE[0].1);
    for &(v, p) in CURVE.iter().skip(1).chain(&[(FULL_MV, 100)]) {
        if mv < v {
            if mv <= prev.0 {
                return prev.1;
            }
            return prev.1 + ((mv - prev.0) as u32 * (p - prev.1) as u32 / (v - prev.0) as u32) as u8;
        }
        prev = (v, p);
    }
    100
}

/// Measures the battery every few seconds into [`MILLIVOLTS`] and [`PERCENT`].
pub async fn run(adc: Peri<'static, peripherals::SAADC>, pin: Peri<'static, peripherals::P0_31>) -> ! {
    let mut config = saadc::Config::default();
    // 16 samples averaged in hardware.
    config.oversample = Oversample::OVER16X;
    let mut channel = ChannelConfig::single_ended(pin);
    // The divider's resistors are large; give the sample-and-hold capacitor time to charge.
    channel.time = Time::_40US;
    let mut adc = Saadc::new(adc, Irqs, config, [channel]);
    // The SoftDevice owns priorities 0, 1 and 4.
    interrupt::SAADC.set_priority(Priority::P3);
    adc.calibrate().await;
    // The first conversion after calibration reads close to zero; drop it.
    adc.sample(&mut [0; 1]).await;

    // In mV × 16, so the filter does not lose the fractions.
    let mut filtered: Option<i32> = None;
    let mut reported: Option<u8> = None;
    loop {
        let mut buf = [0i16; 1];
        adc.sample(&mut buf).await;
        let raw = buf[0].max(0) as u32;
        let mv = (raw * 3600 / 4096 * DIVIDER_X1000 / 1000) as i32;
        // The voltage dips with every radio burst and with the ring's current: take 1/16 of each
        // new reading, which at one reading every 2 s averages over about half a minute.
        let f = match filtered {
            None => mv * 16,
            Some(f) => f + (mv * 16 - f) / 16,
        };
        filtered = Some(f);
        let mv = (f / 16) as u16;
        MILLIVOLTS.store(mv, Ordering::Relaxed);

        // Around 80-90 % one percent is only about 8 mV. Move the reported level only once the
        // voltage is HYSTERESIS_MV past the point where it would change, so it does not flip
        // between neighbours.
        let level = match reported {
            Some(r) if percent(mv) > r => percent(mv.saturating_sub(HYSTERESIS_MV)).max(r),
            Some(r) if percent(mv) < r => percent(mv + HYSTERESIS_MV).min(r),
            Some(r) => r,
            None => percent(mv),
        };
        if reported != Some(level) {
            log!("battery: {} mV, {}%", mv, level);
            reported = Some(level);
            PERCENT.store(level, Ordering::Relaxed);
        }
        Timer::after_secs(2).await;
    }
}
