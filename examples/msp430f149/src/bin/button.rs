//! Toggle an LED from a port interrupt, awaited as a future.
//!
//! The key on P1.4 needs an **external pull-up**: F1xx ports have no internal ones, which is why
//! this asks for [`Pull::None`] rather than [`Pull::Up`]. Asking for a pull on this family panics
//! rather than quietly doing nothing.
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
    let mut button = Input::new(p.P1_4, Pull::None);

    loop {
        button.wait_for_falling_edge().await;
        led.toggle();

        // Crude debounce: ignore whatever else the contact bounce produces.
        Timer::after_millis(50).await;
    }
}
