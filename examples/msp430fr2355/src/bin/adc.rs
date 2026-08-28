//! Read an analog input and print it in millivolts over the backchannel UART.
//!
//! A1 is P1.1. It is measured against the internal 1.5 V reference rather than the supply rail,
//! so the millivolt figure means something without knowing what VCC happens to be — anything above
//! 1.5 V on the pin simply reads full scale.
//!
//! Open the LaunchPad's serial port at 115200 8N1.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::Config;
use embassy_msp430::adc::{Adc, Config as AdcConfig, Reference, SampleTime};
use embassy_msp430::clock::DcoFreq;
use embassy_msp430::uart::{Config as UartConfig, Uart};
use embassy_time::Timer;
use panic_msp430 as _;

/// Four decimal digits, most significant first. Done by hand to keep `core::fmt` out of a 32 kB part.
fn dec4(mut value: u16) -> [u8; 4] {
    let mut out = [b'0'; 4];
    for slot in out.iter_mut().rev() {
        *slot = b'0' + (value % 10) as u8;
        value /= 10;
    }
    out
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    // 115200 baud off the ~1 MHz reset default leaves very little margin, so speed the DCO up.
    config.clock.dco = DcoFreq::_8MHz;
    let p = embassy_msp430::init(config);

    let mut uart = Uart::new(p.EUSCI_A1, p.P4_2, p.P4_3, UartConfig::default()).unwrap();

    let mut adc_config = AdcConfig::default();
    adc_config.reference = Reference::Internal1V5;
    // A long sample window, so a high-impedance source such as a potentiometer still charges the
    // sampling capacitor fully.
    adc_config.sample_time = SampleTime::_256;
    let mut adc = Adc::new(p.ADC, adc_config);

    let mut pin = p.P1_1;

    loop {
        let raw = adc.read(&mut pin).await;
        let mv = adc.to_millivolts(raw).unwrap_or(0);

        uart.write(b"A1: ").await.unwrap();
        uart.write(&dec4(mv)).await.unwrap();
        uart.write(b" mV\r\n").await.unwrap();

        Timer::after_millis(500).await;
    }
}
