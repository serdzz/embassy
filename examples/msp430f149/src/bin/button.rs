//! Toggle an LED from a port interrupt, awaited as a future.
//!
//! The key on P1.4 needs an external pull-up: F1xx ports have no internal ones.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430f149_examples::board;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut board = board::init();

    loop {
        board.button.wait_for_press().await;
        board.led1.toggle();

        // Crude debounce: ignore whatever else the contact bounce produces.
        Timer::after_millis(50).await;
    }
}
