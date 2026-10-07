//! Scroll text on the LaunchPad's on-board segmented LCD with the two buttons.
//!
//! S1 (P1.2) scrolls left, S2 (P2.6) scrolls right — both active-low with a pull-up, same as the
//! out-of-box demo's buttons (see the Energia `pins_energia.h` for this LaunchPad). LED2 (P4.0)
//! keeps blinking independently throughout.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::Peri;
use embassy_msp430::gpio::{Input, Level, Output, Pull};
use embassy_msp430::peripherals::{P1_2, P2_6, P4_0};
use embassy_msp430fr4133_examples::lcd::{Lcd, NUM_CHARS};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use panic_msp430 as _;

/// Which way the text should move, set by whichever button task last fired.
static DIRECTION: Signal<CriticalSectionRawMutex, i8> = Signal::new();

const TEXT: &str = "   EMBASSY ON MSP430FR4133   ";

#[embassy_executor::task]
async fn blink(pin: Peri<'static, P4_0>) {
    let mut led = Output::new(pin, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn button_left(pin: Peri<'static, P1_2>) {
    let mut button = Input::new(pin, Pull::Up);
    loop {
        button.wait_for_falling_edge().await;
        DIRECTION.signal(-1);
        Timer::after_millis(30).await; // crude debounce
        button.wait_for_rising_edge().await;
    }
}

#[embassy_executor::task]
async fn button_right(pin: Peri<'static, P2_6>) {
    let mut button = Input::new(pin, Pull::Up);
    loop {
        button.wait_for_falling_edge().await;
        DIRECTION.signal(1);
        Timer::after_millis(30).await; // crude debounce
        button.wait_for_rising_edge().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_msp430::init(Default::default());

    spawner.spawn(blink(p.P4_0).unwrap());
    spawner.spawn(button_left(p.P1_2).unwrap());
    spawner.spawn(button_right(p.P2_6).unwrap());

    let mut lcd = Lcd::new();
    let max_offset = TEXT.chars().count() - NUM_CHARS;
    let mut offset: usize = 0;
    lcd.show_window(TEXT, offset);

    // Auto-scroll right by default; a button press takes over for one step, then auto-scroll
    // resumes. Holding a button repeats the step every time it re-fires (the debounce above makes
    // each physical press count once).
    loop {
        match embassy_futures::select::select(DIRECTION.wait(), Timer::after(Duration::from_millis(400))).await {
            embassy_futures::select::Either::First(-1) => {
                offset = offset.saturating_sub(1);
            }
            embassy_futures::select::Either::First(_) => {
                offset = (offset + 1).min(max_offset);
            }
            embassy_futures::select::Either::Second(()) => {
                offset = if offset >= max_offset { 0 } else { offset + 1 };
            }
        }
        lcd.show_window(TEXT, offset);
    }
}
