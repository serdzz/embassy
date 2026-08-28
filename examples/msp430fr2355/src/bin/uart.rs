//! Echo characters back over the LaunchPad's backchannel UART, while an LED keeps blinking.
//!
//! `EUSCI_A1` on P4.3/P4.2 is the one wired to the eZ-FET, so it appears as a serial port on the
//! host. Open it at 115200 8N1 and type; characters come back uppercased. The LED carries on
//! blinking throughout, which is the point: neither task blocks the other, and the executor is
//! asleep whenever both are waiting.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::clock::DcoFreq;
use embassy_msp430::gpio::{Level, Output};
use embassy_msp430::peripherals::P1_0;
use embassy_msp430::uart::{Config as UartConfig, Uart};
use embassy_msp430::{Config, Peri};
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
    let mut config = Config::default();
    // 115200 baud off the ~1 MHz reset default leaves very little margin, so speed the DCO up.
    config.clock.dco = DcoFreq::_8MHz;
    let p = embassy_msp430::init(config);

    spawner.spawn(blink(p.P1_0).unwrap());

    let mut uart = Uart::new(p.EUSCI_A1, p.P4_2, p.P4_3, UartConfig::default()).unwrap();
    uart.write(b"embassy-msp430: type something\r\n").await.unwrap();

    let mut buf = [0u8; 1];
    loop {
        uart.read(&mut buf).await.unwrap();
        buf[0] = buf[0].to_ascii_uppercase();
        uart.write(&buf).await.unwrap();
    }
}
