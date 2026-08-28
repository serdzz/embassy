//! Scan the I2C bus and report what answers, over the backchannel UART.
//!
//! I2C is on eUSCI_B0: SCL on P1.3, SDA on P1.2. Both lines need external pull-ups — the eUSCI
//! drives them open-drain and does not pull up, so without resistors the scan finds nothing.
//!
//! Open the LaunchPad's serial port at 115200 8N1 to see the results.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::Config;
use embassy_msp430::clock::DcoFreq;
use embassy_msp430::i2c::{Config as I2cConfig, I2c};
use embassy_msp430::uart::{Config as UartConfig, Uart};
use embassy_time::Timer;
use panic_msp430 as _;

/// Two lowercase hex digits. Done by hand to keep `core::fmt` out of a 32 kB part.
fn hex(byte: u8) -> [u8; 2] {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    [DIGITS[(byte >> 4) as usize], DIGITS[(byte & 0xf) as usize]]
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    // 115200 baud off the ~1 MHz reset default leaves very little margin, so speed the DCO up.
    config.clock.dco = DcoFreq::_8MHz;
    let p = embassy_msp430::init(config);

    let mut uart = Uart::new(p.EUSCI_A1, p.P4_2, p.P4_3, UartConfig::default()).unwrap();
    let mut i2c = I2c::new(p.EUSCI_B0, p.P1_3, p.P1_2, I2cConfig::default()).unwrap();

    loop {
        uart.write(b"scan:").await.unwrap();

        let mut found = 0u8;
        // 0x00..0x07 and 0x78..0x7f are reserved by the I2C specification.
        for address in 0x08..=0x77u8 {
            // A zero-length write puts the address on the bus and stops. An ACK means somebody
            // is listening; a NACK is the silence of an empty slot.
            if i2c.write(address, &[]).await.is_ok() {
                uart.write(b" 0x").await.unwrap();
                uart.write(&hex(address)).await.unwrap();
                found += 1;
            }
        }

        if found == 0 {
            uart.write(b" nothing").await.unwrap();
        }
        uart.write(b"\r\n").await.unwrap();

        Timer::after_secs(2).await;
    }
}
