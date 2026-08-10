#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, bind_interrupts, interrupt};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

/// How long the line has to hold its new level before a press or release is believed.
const DEBOUNCE_MS: u64 = 30;

bind_interrupts!(
    pub struct Irqs{
        EXTI0 => exti::InterruptHandler<interrupt::typelevel::EXTI0>;
});

#[embassy_executor::main(executor = "embassy_stm32::executor::Executor", entry = "cortex_m_rt::entry")]
async fn async_main(_spawner: Spawner) {
    // Delay to allow probe-rs to connect for flashing. Once this example parks in STOP the
    // debugger can no longer attach, so without a window like this the board has to be rescued
    // via BOOT0 to be reprogrammed.
    cortex_m::asm::delay(1_000_000);

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;

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
    // when enabled the power-consumption is much higher during stop, but debugging and RTT is working
    // if you wan't to measure the power-consumption, or for production: uncomment this line
    // config.enable_debug_during_sleep = false;
    let p = embassy_stm32::init(config);

    // KEY on the Black Pill: it shorts PA0 to GND when pressed, so use the internal pull-up.
    let mut button = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up, Irqs);
    // The Black Pill LED is active low: it sits between 3V3 and PC13.
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    info!("Press the USER button...");

    // Nothing else is pending, so the executor puts the chip in STOP between presses;
    // the EXTI line wakes it back up. The LED state survives STOP, so it also shows at a
    // glance that a press was registered without needing a debugger attached.
    loop {
        button.wait_for_falling_edge().await;

        // The contacts chatter, so an edge on its own means nothing: only accept the press if
        // the line is still low once it has had time to settle. Without this the LED toggles
        // several times per press and ends up in an arbitrary state.
        Timer::after_millis(DEBOUNCE_MS).await;
        if button.is_high() {
            info!("button.is_high");
            continue;
        }

        led.toggle();
        info!("Pressed! LED is now {}", if led.is_set_low() { "on" } else { "off" });

        // Same again for the release. This waits on the level rather than a rising edge: a short
        // press is already over by the time the debounce expires, and waiting for an edge that
        // has been and gone would hang here until the *next* press.
        loop {
            button.wait_for_high().await;
            Timer::after_millis(DEBOUNCE_MS).await;
            if button.is_high() {
                break;
            }
        }
    }
}
