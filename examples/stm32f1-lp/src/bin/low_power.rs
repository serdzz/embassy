//! Low-power (STOP mode) example for STM32F1.
//!
//! Blinks an LED on PC13 (the onboard LED of the common "blue pill" board)
//! once every 5 seconds. Between blinks, the low-power executor puts the
//! core into STOP mode, with the RTC alarm (via EXTI line 17) waking it up.
//!
//! The RTC must be clocked from LSE or LSI for wakeup from STOP to work.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::rcc::LsConfig;
use embassy_stm32::{Config, Peri};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main(executor = "embassy_stm32::executor::Executor", entry = "cortex_m_rt::entry")]
async fn async_main(spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.ls = LsConfig::default_lsi();
    // when enabled the power-consumption is much higher during stop, but debugging and RTT is working
    // if you want to measure the power-consumption, or for production: uncomment this line
    // config.enable_debug_during_sleep = false;
    let p = embassy_stm32::init(config);

    info!("low power: starting blinky");

    spawner.spawn(unwrap!(blinky(p.PC13.into())));
}

#[embassy_executor::task]
async fn blinky(led: Peri<'static, AnyPin>) -> ! {
    let mut led = Output::new(led, Level::High, Speed::Low);
    loop {
        info!("blink");
        led.set_low(); // active low on the blue pill
        Timer::after_millis(200).await;

        led.set_high();
        Timer::after_secs(5).await;
    }
}
