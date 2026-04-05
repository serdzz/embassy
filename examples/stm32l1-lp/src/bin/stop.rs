#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::rcc::{Hse, HseMode, LsConfig, Pll, PllDiv, PllMul, PllSource, Sysclk};
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, Peri};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main(executor = "embassy_stm32::executor::Executor", entry = "cortex_m_rt::entry")]
async fn async_main(spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.ls = LsConfig::default_lsi();

    // 8 MHz HSE + PLL → 32 MHz SYSCLK (max for STM32L1)
    // PLL: 8 MHz × 8 = 64 MHz VCO, ÷2 = 32 MHz output
    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll = Some(Pll {
        source: PllSource::HSE,
        mul: PllMul::MUL8,  // 8 MHz × 8 = 64 MHz VCO
        div: PllDiv::DIV2,  // 64 MHz ÷ 2 = 32 MHz
    });
    config.rcc.sys = Sysclk::PLL1_R;

    // when enabled the power-consumption is much higher during stop, but debugging and RTT is working
    // if you want to measure the power-consumption, or for production: uncomment this line
    // config.enable_debug_during_sleep = false;
    let p = embassy_stm32::init(config);

    spawner.spawn(unwrap!(blinky(p.PB6.into())));
    spawner.spawn(unwrap!(timeout()));
}

#[embassy_executor::task]
async fn blinky(led: Peri<'static, AnyPin>) -> ! {
    let mut led = Output::new(led, Level::Low, Speed::Low);
    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(300).await;

        info!("low");
        led.set_low();
        Timer::after_millis(300).await;
    }
}

// when enable_debug_during_sleep is false, it is more difficult to reprogram the MCU
// therefore we block the MCU after 30s to be able to reprogram it easily
#[embassy_executor::task]
async fn timeout() -> ! {
    Timer::after_secs(30).await;
    loop {}
}
