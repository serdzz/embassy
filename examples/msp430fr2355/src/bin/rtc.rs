//! A once-a-second heartbeat driven by the RTC off the VLO, with the executor asleep in LPM3.
//!
//! Nothing here uses `embassy-time`: the RTC has its own prescaler and counter, and the VLO is a
//! separate oscillator, so between blinks the CPU, both main clocks, the DCO and Timer_B0 are all
//! idle. That is about as little as this chip can do while still doing something.
//!
//! The VLO is not trimmed, so "a second" here is a second give or take a good deal. Use XT1 if the
//! interval has to be right.
#![no_std]
#![no_main]

use embassy_executor::{LowPowerMode, Spawner, set_low_power_mode};
use embassy_msp430::gpio::{Level, Output};
use embassy_msp430::rtc::{Config as RtcConfig, Prescaler, Rtc, RtcClock};
use panic_msp430 as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_msp430::init(Default::default());

    let mut led = Output::new(p.P1_0, Level::Low);

    let mut rtc_config = RtcConfig::default();
    rtc_config.clock = RtcClock::Vlo;
    // ~10 kHz / 10, so the counter advances about once a millisecond.
    rtc_config.prescaler = Prescaler::_10;
    let mut rtc = Rtc::new(p.RTC, rtc_config);

    // About 1000 ticks, comfortably inside the 16-bit counter.
    assert!(rtc.set_period_millis(1000));

    set_low_power_mode(LowPowerMode::Lpm3);

    loop {
        rtc.wait().await;
        led.toggle();
    }
}
