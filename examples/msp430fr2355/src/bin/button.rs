//! Toggle an LED from a GPIO interrupt, awaited as a future.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::gpio::{Input, Level, Output, Pull};
use embassy_time::Timer;
use panic_msp430 as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_msp430::init(Default::default());

    let mut led = Output::new(p.P1_0, Level::Low);
    // S1 on the LaunchPad pulls P4.1 to ground when pressed.
    let mut button = Input::new(p.P4_1, Pull::Up);

    loop {
        button.wait_for_falling_edge().await;
        led.toggle();

        // Crude debounce: ignore whatever else the contact bounce produces.
        Timer::after_millis(50).await;
    }
}
