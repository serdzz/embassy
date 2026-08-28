//! The same blink, but with the executor sleeping in LPM3 between timer events.
//!
//! LPM3 stops the CPU, MCLK, SMCLK and the DCO's dc generator; only ACLK keeps running. That is
//! exactly what the Timer_B0 time driver needs, so `embassy-time` keeps working while the core
//! draws single-digit microamps.
//!
//! The mode can be changed at any point, including from a task, so an application can go deeper
//! while a peripheral is idle and come back up before using it again.
#![no_std]
#![no_main]

use embassy_executor::{LowPowerMode, Spawner, set_low_power_mode};
use embassy_msp430::Config;
use embassy_msp430::clock::{AclkSource, DcoFreq};
use embassy_msp430::gpio::{Level, Output};
use embassy_time::Timer;
use panic_msp430 as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    // ACLK off REFO keeps the time driver running in LPM3 without needing a crystal, and the DCO
    // is only woken for the short bursts of work between timer events.
    config.clock.aclk = AclkSource::Refo;
    config.clock.dco = DcoFreq::_8MHz;
    let p = embassy_msp430::init(config);

    set_low_power_mode(LowPowerMode::Lpm3);

    let mut led = Output::new(p.P1_0, Level::Low);
    loop {
        // A short pulse, so most of the period is spent asleep.
        led.set_high();
        Timer::after_millis(20).await;
        led.set_low();
        Timer::after_millis(1980).await;
    }
}
