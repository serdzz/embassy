//! Show text and a counter on the LaunchPad's on-board segmented LCD.
//!
//! LED2 (P4.0) keeps blinking independently, same pattern as the other examples: the LCD write
//! calls don't block it, and vice versa.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::Peri;
use embassy_msp430::gpio::{Level, Output};
use embassy_msp430::peripherals::P4_0;
use embassy_msp430fr4133_examples::lcd::Lcd;
use embassy_time::Timer;
use panic_msp430 as _;

#[embassy_executor::task]
async fn blink(pin: Peri<'static, P4_0>) {
    let mut led = Output::new(pin, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_msp430::init(Default::default());

    spawner.spawn(blink(p.P4_0).unwrap());

    let mut lcd = Lcd::new();
    lcd.show_text("HELLO ");
    Timer::after_millis(2000).await;

    let mut n: u32 = 0;
    let mut digits = [b' '; 6];
    loop {
        Timer::after_millis(1000).await;
        n = (n + 1) % 1_000_000;

        for d in digits.iter_mut() {
            *d = b' ';
        }
        let mut i = digits.len();
        let mut v = n;
        loop {
            i -= 1;
            digits[i] = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }

        for (position, &byte) in digits.iter().enumerate() {
            lcd.show_char(byte as char, position);
        }
    }
}
