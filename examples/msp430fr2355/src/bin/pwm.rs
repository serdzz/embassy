//! Two LEDs breathing in antiphase on one timer.
//!
//! The LaunchPad's own LEDs are on P1.0 and P6.6, neither of which a free timer can drive, so wire
//! LEDs (with series resistors) from P6.0 and P6.1 to ground. They are TB3's channels 1 and 2.
//!
//! Both share TB3's period and differ only in duty, which is the point: one timer, independent
//! outputs.
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_msp430::pwm::{Config as PwmConfig, Pwm};
use embassy_time::Timer;
use panic_msp430 as _;

/// Steps in one direction of the sweep.
const STEPS: u16 = 50;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_msp430::init(Default::default());

    // 1 kHz: far above what the eye can see, and slow enough that the ~1 MHz SMCLK still leaves
    // plenty of counts per period for a smooth sweep.
    let mut pwm = Pwm::new(p.TB3, 1_000, PwmConfig::default()).unwrap();
    let max = pwm.max_duty_cycle();

    let mut up = pwm.channel(p.P6_0);
    let mut down = pwm.channel(p.P6_1);

    loop {
        for step in 0..=STEPS {
            let duty = (max as u32 * step as u32 / STEPS as u32) as u16;
            up.set_duty(duty);
            down.set_duty(max - duty);
            Timer::after_millis(20).await;
        }
        for step in (0..=STEPS).rev() {
            let duty = (max as u32 * step as u32 / STEPS as u32) as u16;
            up.set_duty(duty);
            down.set_duty(max - duty);
            Timer::after_millis(20).await;
        }
    }
}
