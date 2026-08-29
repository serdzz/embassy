//! The converter, the calendar clock and a PWM output, reporting over the UART once a minute.
//!
//! Between reports everything is asleep: the RTC's own minute event is what wakes the machine, so
//! the CPU does no work at all in the meantime. That is the shape a meter has — measure, record,
//! sleep — and it exercises four drivers at once.
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_msp430::adc::{Adc, Channel, Config as AdcConfig, Reference, SampleTime};
use embassy_msp430::clock::DcoFreq;
use embassy_msp430::pwm::{Config as PwmConfig, Pwm};
use embassy_msp430::rtc_c::{ClockSource, DateTime, Interval, Rtc};
use embassy_msp430::uart::{Config as UartConfig, Uart};

use panic_msp430 as _;

/// Two decimal digits. Written out rather than reached for through `core::fmt`, which costs a
/// kilobyte or so on a part with forty of them.
fn dec2(value: u8) -> [u8; 2] {
    [b'0' + (value / 10) % 10, b'0' + value % 10]
}

/// Four decimal digits, most significant first.
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
    let mut config = embassy_msp430::Config::default();
    config.clock.dco = DcoFreq::_8MHz;
    let p = embassy_msp430::init(config);

    let mut uart = Uart::new(p.EUSCI_A0, p.P4_4, p.P4_3, UartConfig::default()).unwrap();

    // The internal reference, because a reading against AVCC is only as good as the battery.
    let mut adc_config = AdcConfig::default();
    adc_config.reference = Reference::Internal2V0;
    adc_config.sample_time = SampleTime::_64;
    let mut adc = Adc::new(p.ADC12, adc_config);

    // Something for the PWM to drive: TA0.0 on P5.4, at 1 kHz, a quarter on. P2.3 carries the same
    // channel and is the obvious choice on an 80-pin FR6043 -- but it is not bonded out on the
    // FR5043's 64-pin package, and this example builds for both.
    let mut pwm = Pwm::new(p.TA0, 1_000, PwmConfig::default()).unwrap();
    let mut channel = pwm.channel(p.P5_4);
    let quarter = channel.max_duty_cycle() / 4;
    channel.set_duty(quarter);

    let mut rtc = Rtc::new(
        p.RTC,
        ClockSource::Xt1,
        DateTime {
            year: 2026,
            month: 8,
            day: 29,
            weekday: 6,
            hour: 12,
            minute: 0,
            second: 0,
        },
    )
    .unwrap();

    loop {
        let now = rtc.now();
        // Half the supply rail, doubled, is the supply. Only meaningful against a known reference,
        // which is why this example asks for the internal one.
        let raw = adc.read_internal(Channel::HalfAvcc).await;
        let millivolts = adc.to_millivolts(raw).unwrap_or(0) * 2;

        uart.write(&dec2(now.hour)).await.unwrap();
        uart.write(b":").await.unwrap();
        uart.write(&dec2(now.minute)).await.unwrap();
        uart.write(b"  supply ").await.unwrap();
        uart.write(&dec4(millivolts)).await.unwrap();
        uart.write(b" mV\r\n").await.unwrap();

        rtc.wait_interval(Interval::Minute).await;
    }
}
