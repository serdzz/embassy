//! SPI loopback self-test on eUSCI_B0.
//!
//! Wire P1.2 (MOSI) to P1.3 (MISO) with a jumper. Every byte sent then comes straight back, so the
//! test can tell a working bus from a silent one without any device attached.
//!
//! LED1 blinks quickly while the loopback matches and slowly when it does not — pull the jumper
//! out and the blink rate drops.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::gpio::{Level, Output};
use embassy_msp430::spi::{Config as SpiConfig, Spi};
use embassy_time::Timer;
use panic_msp430 as _;

const PATTERN: [u8; 4] = [0xA5, 0x5A, 0x00, 0xFF];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_msp430::init(Default::default());

    let mut led = Output::new(p.P1_0, Level::Low);
    let mut spi = Spi::new(p.EUSCI_B0, p.P1_1, p.P1_2, p.P1_3, SpiConfig::default()).unwrap();

    loop {
        let mut buf = PATTERN;
        let ok = spi.transfer_in_place(&mut buf).await.is_ok() && buf == PATTERN;

        let period = if ok { 60 } else { 400 };
        for _ in 0..4 {
            led.toggle();
            Timer::after_millis(period).await;
        }
    }
}
