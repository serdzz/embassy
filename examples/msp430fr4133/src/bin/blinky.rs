//! Two LEDs blinking at different rates from two independent tasks.
//!
//! LED1 is on P1.0 and LED2 on P4.0 on the MSP-EXP430FR4133 LaunchPad.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::Peri;
use embassy_msp430::gpio::{Level, Output};
use embassy_msp430::peripherals::P4_0;
use embassy_time::Timer;
use panic_msp430 as _;

#[embassy_executor::task]
async fn heartbeat(pin: Peri<'static, P4_0>) {
    let mut led = Output::new(pin, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_msp430::init(Default::default());

    spawner.spawn(heartbeat(p.P4_0).unwrap());

    let mut led = Output::new(p.P1_0, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(133).await;
    }
}
