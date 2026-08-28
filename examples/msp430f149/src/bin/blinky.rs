//! Two LEDs blinking at different rates from two independent tasks.
//!
//! LEDs on P1.0 and P1.1 — check that against your own board, F149 boards are not standardised.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::Peri;
use embassy_msp430::gpio::{Level, Output};
use embassy_msp430::peripherals::P1_1;
use embassy_time::Timer;
use panic_msp430 as _;

#[embassy_executor::task]
async fn heartbeat(pin: Peri<'static, P1_1>) {
    let mut led = Output::new(pin, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_msp430::init(Default::default());

    spawner.spawn(heartbeat(p.P1_1).unwrap());

    let mut led = Output::new(p.P1_0, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(133).await;
    }
}
