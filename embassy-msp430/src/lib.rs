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

#[cfg(not(any(
    feature = "msp430fr2355",
    feature = "msp430f149",
    feature = "msp430f2618",
    feature = "msp430fr6043"
)))]
compile_error!("No chip selected. Enable exactly one chip feature, e.g. `msp430fr2355`.");

#[cfg(any(
    all(feature = "msp430fr2355", feature = "msp430f149"),
    all(feature = "msp430fr2355", feature = "msp430f2618"),
    all(feature = "msp430fr2355", feature = "msp430fr6043"),
    all(feature = "msp430f149", feature = "msp430f2618"),
    all(feature = "msp430f149", feature = "msp430fr6043"),
    all(feature = "msp430f2618", feature = "msp430fr6043")
))]
compile_error!("More than one chip feature is enabled. Enable exactly one.");

pub(crate) mod chip;

/// Peripheral access crate for the selected chip.
#[cfg(feature = "msp430f149")]
pub use msp430f149_pac as pac;
/// Peripheral access crate for the selected chip.
#[cfg(feature = "msp430f2618")]
pub use msp430f2618_pac as pac;
/// Peripheral access crate for the selected chip.
#[cfg(feature = "msp430fr2355")]
pub use msp430fr2355_pac as pac;
/// Peripheral access crate for the selected chip.
#[cfg(feature = "msp430fr6043")]
pub use msp430fr6043_pac as pac;
pub use {embassy_executor, msp430, msp430_rt};

pub mod clock;
pub mod gpio;
pub mod wdt;

// The FR2xx serial and analog peripherals share nothing with the F1xx ones below the pin, so these
// are for now only available on the family they were written against.
#[cfg(any(
    feature = "msp430fr2355",
    feature = "msp430f149",
    feature = "msp430fr6043"
))]
pub mod adc;
#[cfg(any(feature = "msp430fr2355", feature = "msp430fr6043"))]
pub(crate) mod eusci;
#[cfg(any(feature = "msp430fr2355", feature = "msp430fr6043"))]
pub mod i2c;
#[cfg(any(
    feature = "msp430fr2355",
    feature = "msp430f149",
    feature = "msp430fr6043"
))]
pub mod pwm;
#[cfg(feature = "msp430fr2355")]
pub mod rtc;
#[cfg(feature = "msp430fr6043")]
pub mod rtc_c;
#[cfg(any(
    feature = "msp430fr2355",
    feature = "msp430f149",
    feature = "msp430fr6043"
))]
pub mod spi;
#[cfg(any(
    feature = "msp430fr2355",
    feature = "msp430f149",
    feature = "msp430fr6043"
))]
pub mod uart;
#[cfg(feature = "msp430f149")]
pub(crate) mod usart;

#[cfg(feature = "_time-driver")]
mod time_driver;

pub use embassy_hal_internal::{Peri, PeripheralType};

#[cfg(feature = "msp430fr2355")]
embassy_hal_internal::peripherals! {
    P1_0, P1_1, P1_2, P1_3, P1_4, P1_5, P1_6, P1_7,
    P2_0, P2_1, P2_2, P2_3, P2_4, P2_5, P2_6, P2_7,
    P3_0, P3_1, P3_2, P3_3, P3_4, P3_5, P3_6, P3_7,
    P4_0, P4_1, P4_2, P4_3, P4_4, P4_5, P4_6, P4_7,
    P5_0, P5_1, P5_2, P5_3, P5_4,
    P6_0, P6_1, P6_2, P6_3, P6_4, P6_5, P6_6,

    // The time driver's timer is missing on purpose: it is not the user's to take.
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

#[cfg(feature = "msp430f149")]
embassy_hal_internal::peripherals! {
    P1_0, P1_1, P1_2, P1_3, P1_4, P1_5, P1_6, P1_7,
    P2_0, P2_1, P2_2, P2_3, P2_4, P2_5, P2_6, P2_7,
    P3_0, P3_1, P3_2, P3_3, P3_4, P3_5, P3_6, P3_7,
    P4_0, P4_1, P4_2, P4_3, P4_4, P4_5, P4_6, P4_7,
    P5_0, P5_1, P5_2, P5_3, P5_4, P5_5, P5_6, P5_7,
    P6_0, P6_1, P6_2, P6_3, P6_4, P6_5, P6_6, P6_7,

    // The time driver's timer is missing on purpose: it is not the user's to take.
    #[cfg(not(feature = "time-driver-tb0"))]
    TB0,
    TA0,

    USART0,
    USART1,

    ADC12,
    MPY,
}

#[cfg(feature = "msp430f2618")]
embassy_hal_internal::peripherals! {
    P1_0, P1_1, P1_2, P1_3, P1_4, P1_5, P1_6, P1_7,
    P2_0, P2_1, P2_2, P2_3, P2_4, P2_5, P2_6, P2_7,
    P3_0, P3_1, P3_2, P3_3, P3_4, P3_5, P3_6, P3_7,
    P4_0, P4_1, P4_2, P4_3, P4_4, P4_5, P4_6, P4_7,
    P5_0, P5_1, P5_2, P5_3, P5_4, P5_5, P5_6, P5_7,
    P6_0, P6_1, P6_2, P6_3, P6_4, P6_5, P6_6, P6_7,
    P7_0, P7_1, P7_2, P7_3, P7_4, P7_5, P7_6, P7_7,
    P8_0, P8_1, P8_2, P8_3, P8_4, P8_5, P8_6, P8_7,

    // The time driver's timer is missing on purpose: it is not the user's to take.
    #[cfg(not(feature = "time-driver-tb0"))]
    TB0,
    TA0,

    USCI_A0,
    USCI_A1,
    USCI_B0,
    USCI_B1,

    ADC12,
    DAC12,
    MPY,
}

#[cfg(feature = "msp430fr6043")]
embassy_hal_internal::peripherals! {
    P1_0, P1_1, P1_2, P1_3, P1_4, P1_5, P1_6, P1_7,
    P2_0, P2_1, P2_2, P2_3, P2_4, P2_5, P2_6, P2_7,
    P3_0, P3_1, P3_2, P3_3, P3_4, P3_5, P3_6, P3_7,
    P4_0, P4_1, P4_2, P4_3, P4_4, P4_5, P4_6, P4_7,
    P5_0, P5_1, P5_2, P5_3, P5_4, P5_5, P5_6, P5_7,
    P6_0, P6_1, P6_2, P6_3, P6_4, P6_5, P6_6, P6_7,
    P7_0, P7_1, P7_2, P7_3, P7_4, P7_5, P7_6, P7_7,
    P8_0, P8_1, P8_2, P8_3, P8_4, P8_5, P8_6, P8_7,
    P9_0, P9_1, P9_2, P9_3, P9_4, P9_5, P9_6, P9_7,

    // The time driver's timer is missing on purpose: it is not the user's to take.
    #[cfg(not(feature = "time-driver-tb0"))]
    TB0,
    TA0,
    TA1,
    TA2,
    TA3,
    TA4,

    EUSCI_A0,
    EUSCI_A1,
    EUSCI_A2,
    EUSCI_A3,
    EUSCI_B0,
    EUSCI_B1,

    ADC12,
    RTC,
    LCD,
    COMP,
    DMA,
    MPY32,
    CRC32,
    AES256,

    // The ultrasonic front end. No drivers yet; they are singletons so that whoever writes one — or
    // hands them to TI's library — is the only one holding them.
    SAPH,
    SDHS,
    UUPS,
    HSPLL,
    LEA,
    MTIF,
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

// The FR2355's P5 and P6 stop short of eight pins; the F149's do not.
#[cfg(any(
    feature = "msp430f149",
    feature = "msp430f2618",
    feature = "msp430fr6043"
))]
gpio::impl_pin!(P5_5, 4, 5);
#[cfg(any(
    feature = "msp430f149",
    feature = "msp430f2618",
    feature = "msp430fr6043"
))]
gpio::impl_pin!(P5_6, 4, 6);
#[cfg(any(
    feature = "msp430f149",
    feature = "msp430f2618",
    feature = "msp430fr6043"
))]
gpio::impl_pin!(P5_7, 4, 7);
#[cfg(any(
    feature = "msp430f149",
    feature = "msp430f2618",
    feature = "msp430fr6043"
))]
gpio::impl_pin!(P6_7, 5, 7);

#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P7_0, 6, 0);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P7_1, 6, 1);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P7_2, 6, 2);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P7_3, 6, 3);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P7_4, 6, 4);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P7_5, 6, 5);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P7_6, 6, 6);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P7_7, 6, 7);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P8_0, 7, 0);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P8_1, 7, 1);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P8_2, 7, 2);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P8_3, 7, 3);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P8_4, 7, 4);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P8_5, 7, 5);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P8_6, 7, 6);
#[cfg(any(feature = "msp430f2618", feature = "msp430fr6043"))]
gpio::impl_pin!(P8_7, 7, 7);

#[cfg(feature = "msp430fr6043")]
gpio::impl_pin!(P9_0, 8, 0);
#[cfg(feature = "msp430fr6043")]
gpio::impl_pin!(P9_1, 8, 1);
#[cfg(feature = "msp430fr6043")]
gpio::impl_pin!(P9_2, 8, 2);
#[cfg(feature = "msp430fr6043")]
gpio::impl_pin!(P9_3, 8, 3);
#[cfg(feature = "msp430fr6043")]
gpio::impl_pin!(P9_4, 8, 4);
#[cfg(feature = "msp430fr6043")]
gpio::impl_pin!(P9_5, 8, 5);
#[cfg(feature = "msp430fr6043")]
gpio::impl_pin!(P9_6, 8, 6);
#[cfg(feature = "msp430fr6043")]
gpio::impl_pin!(P9_7, 8, 7);

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
/// This stops the watchdog, applies `config` to the clock system, and starts the `embassy-time`
/// driver if one is enabled. On the FRAM parts it also releases the I/O from the low-power lock the
/// chip boots into.
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

        // The FRAM parts boot with every pin held in the high-impedance state it had in LPM4, and
        // nothing written to a port register takes effect until LOCKLPM5 is cleared. The flash
        // parts have no such lock.
        #[cfg(any(feature = "msp430fr2355", feature = "msp430fr6043"))]
        // SAFETY: single volatile write to PM5CTL0.
        unsafe { pac::Pmm::steal() }
            .pm5ctl0()
            .modify(|_, w| w.locklpm5().locklpm5_0());

        #[cfg(feature = "_time-driver")]
        time_driver::init(cs);

        Peripherals::take_with_cs(cs)
    })
}
