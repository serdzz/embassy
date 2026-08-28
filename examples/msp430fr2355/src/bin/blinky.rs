//! Two LEDs blinking at different rates from two independent tasks.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430fr2355_examples::board::{self, Led2};
use embassy_time::Timer;

#[embassy_executor::task]
async fn heartbeat(mut led: Led2) {
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = board::init();

    spawner.spawn(heartbeat(board.led2).unwrap());

    let mut led = board.led1;
    loop {
        led.toggle();
        Timer::after_millis(133).await;
    }
}
