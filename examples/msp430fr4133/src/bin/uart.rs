//! UART logger over the LaunchPad's backchannel: LED2 keeps blinking while a counter is printed.
//!
//! `EUSCI_A0` on P1.0/P1.1 is the pair wired to the on-board eZ-FET, so it shows up as a second
//! serial port on the host (next to the debug port) once mspdebug/the kernel enumerate the FET.
//! Open it at 9600 8N1 (e.g. `screen /dev/ttyACM1 9600` on Linux) to see the log. Note P1.0 is
//! also LED1 in the `blinky` example — the two are mutually exclusive, so this example drives
//! only LED2 (P4.0) and leaves P1.0 to the UART.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::Peri;
use embassy_msp430::gpio::{Level, Output};
use embassy_msp430::peripherals::P4_0;
use embassy_msp430::uart::{Config as UartConfig, Uart};
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

    let mut uart = Uart::new(p.EUSCI_A0, p.P1_1, p.P1_0, UartConfig::default()).unwrap();
    uart.write(b"\r\nMSP430FR4133 UART logger up\r\n").await.unwrap();

    let mut n: u32 = 0;
    let mut digits = [0u8; 10];
    loop {
        Timer::after_millis(1000).await;
        n = n.wrapping_add(1);

        // No heap, no format! on this target: render the counter by hand.
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

        uart.write(b"tick #").await.unwrap();
        uart.write(&digits[i..]).await.unwrap();
        uart.write(b"\r\n").await.unwrap();
    }
}
