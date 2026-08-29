//! Blink an LED on the MSP430FR6043.
//!
//! The smallest thing that exercises the whole stack: the clock system, the FRAM wait states that
//! go with it, the `embassy-time` driver on Timer_B0, the executor and its low-power sleep, and
//! GPIO. What it is really for is measuring what that costs — see the crate README, because on a
//! part where only 40 kB of the FRAM is reachable the answer decides what else fits.
//!
//! Nothing here touches the ultrasonic front end. There is no driver for it yet.

#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Executor;
use embassy_msp430::clock::{AclkSource, DcoFreq, Div};
use embassy_msp430::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

use panic_msp430 as _;

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[embassy_executor::task]
async fn blink(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[msp430_rt::entry]
fn main() -> ! {
    let mut config = embassy_msp430::Config::default();
    // 8 MHz is the fastest the DCO reaches with no FRAM wait states, so it is the natural default:
    // above it every instruction fetch from FRAM costs an extra cycle.
    config.clock.dco = DcoFreq::_8MHz;
    config.clock.mclk_div = Div::_1;
    config.clock.smclk_div = Div::_1;
    // The watch crystal keeps the time driver running while the CPU is asleep.
    config.clock.aclk = AclkSource::Lfxt;
    let p = embassy_msp430::init(config);

    let led = Output::new(p.P1_0, Level::Low);

    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(blink(led).unwrap());
    })
}
