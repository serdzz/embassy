#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, Peri};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main(executor = "embassy_stm32::executor::Executor", entry = "cortex_m_rt::entry")]
async fn async_main(spawner: Spawner) {
    // Delay to allow probe-rs to connect for flashing. Once this example parks in STOP the
    // debugger can no longer attach, so without a window like this the board has to be rescued
    // via BOOT0 to be reprogrammed.
    cortex_m::asm::delay(1_000_000);

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;

        // The RTC wakes the chip from STOP, so it needs a low-speed clock. The Black Pill has a
        // 32.768 kHz crystal fitted, which is far more accurate than the LSI.
        config.rcc.ls = LsConfig::default_lse();

        // 25 MHz crystal on X1.
        config.rcc.hse = Some(Hse {
            freq: Hertz(25_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV25,  // 25 MHz / 25 = 1 MHz PLL input
            mul: PllMul::MUL400,       // 1 MHz * 400 = 400 MHz VCO
            divp: Some(PllPDiv::DIV4), // 400 MHz / 4 = 100 MHz, the F411 maximum
            divq: None,
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.ahb_pre = AHBPrescaler::DIV1; // HCLK  100 MHz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // PCLK1  50 MHz, the APB1 maximum
        config.rcc.apb2_pre = APBPrescaler::DIV1; // PCLK2 100 MHz
    }
    // The F4 always wakes from STOP on the HSI. Coming back up on the HSE and PLL needs
    // `exit_stop` to restore the saved clock tree, which embassy-stm32 previously only did for
    // the L0/WL/WBA -- see the `stm32f4` branch added to `low_power.rs`.
    // when enabled the power-consumption is much higher during stop, but debugging and RTT is working
    // if you wan't to measure the power-consumption, or for production: uncomment this line
    // config.enable_debug_during_sleep = false;
    let p = embassy_stm32::init(config);

    spawner.spawn(unwrap!(blinky(p.PC13.into())));
    spawner.spawn(unwrap!(timeout()));
}

#[embassy_executor::task]
async fn blinky(led: Peri<'static, AnyPin>) -> ! {
    // The Black Pill LED is active low: it sits between 3V3 and PC13.
    let mut led = Output::new(led, Level::High, Speed::Low);
    loop {
        info!("on");
        led.set_low();
        Timer::after_millis(300).await;

        info!("off");
        led.set_high();
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
