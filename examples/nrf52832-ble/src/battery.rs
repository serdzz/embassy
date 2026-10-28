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
    (4110, 90),
];
const FULL_MV: u16 = 4200;

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

    let mut filtered: Option<u32> = None;
    let mut first = true;
    loop {
        let mut buf = [0i16; 1];
        adc.sample(&mut buf).await;
        let raw = buf[0].max(0) as u32;
        let mv = raw * 3600 / 4096 * DIVIDER_X1000 / 1000;
        // Smooth out the dips from radio bursts and LEDs: 1/4 of each new reading.
        let mv = match filtered {
            None => mv,
            Some(f) => (f * 3 + mv) / 4,
        };
        filtered = Some(mv);
        MILLIVOLTS.store(mv as u16, Ordering::Relaxed);
        let level = percent(mv as u16);
        if PERCENT.swap(level, Ordering::Relaxed) != level || first {
            log!("battery: {} mV, {}%", mv, level);
        }
        first = false;
        Timer::after_secs(2).await;
    }
}
