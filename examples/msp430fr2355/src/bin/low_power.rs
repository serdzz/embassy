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
use embassy_msp430fr2355_examples::board;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut board = board::init();

    set_low_power_mode(LowPowerMode::Lpm3);

    loop {
        // A short pulse, so most of the period is spent asleep.
        board.led1.set(true);
        Timer::after_millis(20).await;
        board.led1.set(false);
        Timer::after_millis(1980).await;
    }
}
