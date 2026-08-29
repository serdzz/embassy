//! Echo characters over `eUSCI_A0`, while an LED keeps blinking.
//!
//! On the 80-pin part `UCA0TXD` comes out on both P2.6 and P4.3; this uses P4.3/P4.4, the pair
//! where both lines are the pin's first alternate function. The LED carries on blinking throughout,
//! which is the point: neither task blocks the other, and the executor is asleep whenever both are
//! waiting.
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_msp430::clock::DcoFreq;
use embassy_msp430::gpio::{Level, Output};
use embassy_msp430::peripherals::P1_0;
use embassy_msp430::uart::{Config as UartConfig, Uart};
use embassy_msp430::Peri;
use embassy_time::Timer;

use panic_msp430 as _;

#[embassy_executor::task]
async fn blink(pin: Peri<'static, P1_0>) {
    let mut led = Output::new(pin, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(250).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_msp430::Config::default();
    // 115200 baud off the 1 MHz reset default leaves very little margin, so speed the DCO up. 8 MHz
    // is the fastest that needs no FRAM wait states.
    config.clock.dco = DcoFreq::_8MHz;
    let p = embassy_msp430::init(config);

    spawner.spawn(blink(p.P1_0).unwrap());

    let mut uart = Uart::new(p.EUSCI_A0, p.P4_4, p.P4_3, UartConfig::default()).unwrap();
    uart.write(b"embassy-msp430 on FR6043: type something\r\n")
        .await
        .unwrap();

    let mut buf = [0u8; 1];
    loop {
        uart.read(&mut buf).await.unwrap();
        buf[0] = buf[0].to_ascii_uppercase();
        uart.write(&buf).await.unwrap();
    }
}
