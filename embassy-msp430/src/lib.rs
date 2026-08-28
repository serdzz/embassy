#![no_std]
// MSP430 is a tier 3 target: the interrupt ABI and inline assembly are still unstable there, and
// `core` has to be built from source. This crate therefore requires a nightly compiler.
#![feature(abi_msp430_interrupt, asm_experimental_arch)]
#![allow(clippy::new_without_default)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

//! ## Feature flags
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

#[cfg(not(feature = "msp430fr2355"))]
compile_error!("No chip selected. Enable exactly one chip feature, e.g. `msp430fr2355`.");

/// Peripheral access crate for the selected chip.
#[cfg(feature = "msp430fr2355")]
pub use msp430fr2355_pac as pac;
pub use {embassy_executor, msp430, msp430_rt};

pub mod adc;
pub mod clock;
pub(crate) mod eusci;
pub mod gpio;
pub mod i2c;
pub mod pwm;
pub mod rtc;
pub mod spi;
pub mod uart;
pub mod wdt;

#[cfg(feature = "_time-driver")]
mod time_driver;

pub use embassy_hal_internal::{Peri, PeripheralType};

embassy_hal_internal::peripherals! {
    P1_0, P1_1, P1_2, P1_3, P1_4, P1_5, P1_6, P1_7,
    P2_0, P2_1, P2_2, P2_3, P2_4, P2_5, P2_6, P2_7,
    P3_0, P3_1, P3_2, P3_3, P3_4, P3_5, P3_6, P3_7,
    P4_0, P4_1, P4_2, P4_3, P4_4, P4_5, P4_6, P4_7,
    P5_0, P5_1, P5_2, P5_3, P5_4,
    P6_0, P6_1, P6_2, P6_3, P6_4, P6_5, P6_6,

    // TB0 is missing on purpose when it is the `embassy-time` driver: it is not the user's to take.
    #[cfg(not(feature = "time-driver-tb0"))]
    TB0,
    TB1,
    TB2,
    TB3,

    EUSCI_A0,
    EUSCI_A1,
    EUSCI_B0,
    EUSCI_B1,

    ADC,
    RTC,
    CRC,
    MPY32,
}

gpio::impl_pin!(P1_0, 0, 0);
gpio::impl_pin!(P1_1, 0, 1);
gpio::impl_pin!(P1_2, 0, 2);
gpio::impl_pin!(P1_3, 0, 3);
gpio::impl_pin!(P1_4, 0, 4);
gpio::impl_pin!(P1_5, 0, 5);
gpio::impl_pin!(P1_6, 0, 6);
gpio::impl_pin!(P1_7, 0, 7);
gpio::impl_pin!(P2_0, 1, 0);
gpio::impl_pin!(P2_1, 1, 1);
gpio::impl_pin!(P2_2, 1, 2);
gpio::impl_pin!(P2_3, 1, 3);
gpio::impl_pin!(P2_4, 1, 4);
gpio::impl_pin!(P2_5, 1, 5);
gpio::impl_pin!(P2_6, 1, 6);
gpio::impl_pin!(P2_7, 1, 7);
gpio::impl_pin!(P3_0, 2, 0);
gpio::impl_pin!(P3_1, 2, 1);
gpio::impl_pin!(P3_2, 2, 2);
gpio::impl_pin!(P3_3, 2, 3);
gpio::impl_pin!(P3_4, 2, 4);
gpio::impl_pin!(P3_5, 2, 5);
gpio::impl_pin!(P3_6, 2, 6);
gpio::impl_pin!(P3_7, 2, 7);
gpio::impl_pin!(P4_0, 3, 0);
gpio::impl_pin!(P4_1, 3, 1);
gpio::impl_pin!(P4_2, 3, 2);
gpio::impl_pin!(P4_3, 3, 3);
gpio::impl_pin!(P4_4, 3, 4);
gpio::impl_pin!(P4_5, 3, 5);
gpio::impl_pin!(P4_6, 3, 6);
gpio::impl_pin!(P4_7, 3, 7);
gpio::impl_pin!(P5_0, 4, 0);
gpio::impl_pin!(P5_1, 4, 1);
gpio::impl_pin!(P5_2, 4, 2);
gpio::impl_pin!(P5_3, 4, 3);
gpio::impl_pin!(P5_4, 4, 4);
gpio::impl_pin!(P6_0, 5, 0);
gpio::impl_pin!(P6_1, 5, 1);
gpio::impl_pin!(P6_2, 5, 2);
gpio::impl_pin!(P6_3, 5, 3);
gpio::impl_pin!(P6_4, 5, 4);
gpio::impl_pin!(P6_5, 5, 5);
gpio::impl_pin!(P6_6, 5, 6);

/// HAL configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// Clock system configuration.
    pub clock: clock::Config,
}

/// Frequencies the clock system was configured for by the last [`init`].
///
/// Peripheral drivers read this to work out their own dividers.
static CLOCKS: critical_section::Mutex<core::cell::Cell<Option<clock::Clocks>>> =
    critical_section::Mutex::new(core::cell::Cell::new(None));

/// Frequencies the clock system is running at, or `None` before [`init`].
pub fn clocks() -> Option<clock::Clocks> {
    critical_section::with(|cs| CLOCKS.borrow(cs).get())
}

/// Initialise the HAL and take the peripheral singletons.
///
/// This stops the watchdog, applies `config` to the clock system, releases the I/O from the
/// low-power lock the chip boots into, and starts the `embassy-time` driver if one is enabled.
///
/// Panics if called more than once.
pub fn init(config: Config) -> Peripherals {
    critical_section::with(|cs| {
        // The watchdog runs out of reset and would reset the chip after about 32 ms.
        wdt::stop();

        // SAFETY: nothing else has been configured yet, so no peripheral can be upset by MCLK
        // changing underneath it.
        let clocks = unsafe { clock::init(config.clock) };
        CLOCKS.borrow(cs).set(Some(clocks));

        // The FRAM parts boot with every pin held in the high-impedance state it had in LPM4.
        // Nothing written to a port register takes effect until LOCKLPM5 is cleared.
        // SAFETY: single volatile write to PM5CTL0.
        unsafe { pac::Pmm::steal() }
            .pm5ctl0()
            .modify(|_, w| w.locklpm5().locklpm5_0());

        #[cfg(feature = "_time-driver")]
        time_driver::init(cs);

        Peripherals::take_with_cs(cs)
    })
}
