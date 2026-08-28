//! Echo characters back over USART0, while an LED keeps blinking.
//!
//! USART0's UART pins are P3.4 (transmit) and P3.5 (receive). Most F149 boards bring them out to a
//! level shifter or a USB-serial header; check yours.
//!
//! The bit clock is ACLK, the watch crystal, which divides to 9600 baud almost exactly. SMCLK
//! would go faster, but on this family it is an untrimmed DCO, so the baud rate would only be as
//! right as the figure given to `clock::Config::dco_hz`.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::Peri;
use embassy_msp430::clock::PeripheralClock;
use embassy_msp430::gpio::{Level, Output};
use embassy_msp430::peripherals::P1_1;
use embassy_msp430::uart::{Config as UartConfig, Uart};
use embassy_time::Timer;
use panic_msp430 as _;

#[embassy_executor::task]
async fn blink(pin: Peri<'static, P1_1>) {
    let mut led = Output::new(pin, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(250).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_msp430::init(Default::default());

    spawner.spawn(blink(p.P1_1).unwrap());

    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 9600;
    uart_config.clock_source = PeripheralClock::Aclk;
    let mut uart = Uart::new(p.USART0, p.P3_5, p.P3_4, uart_config).unwrap();

    uart.write(b"embassy-msp430 on an F149: type something\r\n")
        .await
        .unwrap();

    let mut buf = [0u8; 1];
    loop {
        uart.read(&mut buf).await.unwrap();
        buf[0] = buf[0].to_ascii_uppercase();
        uart.write(&buf).await.unwrap();
    }
}
