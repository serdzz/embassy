#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::rcc::{Hse, HseMode, LsConfig, Pll, PllDiv, PllMul, PllSource, Sysclk};
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, Peri};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

/// How often to wake up from Stop mode.
const WAKEUP_INTERVAL: Duration = Duration::from_secs(5);

#[embassy_executor::main(executor = "embassy_stm32::executor::Executor", entry = "cortex_m_rt::entry")]
async fn async_main(spawner: Spawner) {
    let mut config = Config::default();

    // LSE (32.768 kHz crystal) as RTC clock source.
    config.rcc.ls = LsConfig::default_lse();

    // 8 MHz HSE + PLL → 32 MHz SYSCLK
    // PLL: 8 MHz × 8 = 64 MHz VCO, ÷2 = 32 MHz output
    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll = Some(Pll {
        source: PllSource::HSE,
        mul: PllMul::MUL8,
        div: PllDiv::DIV2,
    });
    config.rcc.sys = Sysclk::PLL1_R;
    // Enable debug during sleep so the debugger can connect even in Stop mode.
    // Set to `false` for real power measurements.
    //config.enable_debug_during_sleep = true;
    let p = embassy_stm32::init(config);

    spawner.spawn(unwrap!(wakeup_task(p.PB6.into())));
    // After 60s the chip stays awake permanently, making it easy to re-flash.
    spawner.spawn(unwrap!(stay_awake_timeout()));
}

#[embassy_executor::task]
async fn wakeup_task(led: Peri<'static, AnyPin>) -> ! {
    let mut led = Output::new(led, Level::Low, Speed::Low);
    let mut count: u32 = 0;
    loop {
        // Entered Stop mode; will wake here after WAKEUP_INTERVAL via RTC wakeup alarm.
        Timer::after(WAKEUP_INTERVAL).await;

        count += 1;
        info!("woke up #{} (every {} s)", count, WAKEUP_INTERVAL.as_secs());

        // Blink LED briefly to indicate wakeup.
        led.set_high();
        Timer::after_millis(50).await;
        led.set_low();
    }
}

// When enable_debug_during_sleep is false, it is harder to reprogram the MCU.
// After 60 s this task spins the executor, keeping the chip awake for easy re-flashing.
#[embassy_executor::task]
async fn stay_awake_timeout() -> ! {
    Timer::after_secs(30).await;
    loop {}
}
