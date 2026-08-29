//! Pulse-width modulation, on the Timer_B blocks.
//!
//! One timer sets the period for all of its channels; each channel then has its own duty cycle.
//!
//! ```ignore
//! let mut pwm = Pwm::new(p.TB1, 1_000, pwm::Config::default())?;
//! let mut led = pwm.channel(p.P2_0);
//! led.set_duty_cycle_percent(25);
//! ```
//!
//! Keep the [`Pwm`] alive for as long as its channels: it owns the timer, and dropping it stops
//! the counter and freezes every output.

use core::convert::Infallible;
use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};

use crate::clock::PeripheralClock;
use crate::gpio::{self, AnyPin, Pin, PinFunction};
use crate::peripherals;

// Control register.
const TBCLR: u16 = 0x0004;
const MC_UP: u16 = 0x0010;
const MC_STOP: u16 = 0x0000;
const ID_SHIFT: u16 = 6;
const TBSSEL_ACLK: u16 = 0x0100;
const TBSSEL_SMCLK: u16 = 0x0200;

// Capture/compare control register.
const OUT: u16 = 0x0004;
const OUTMOD_SHIFT: u16 = 5;
/// Output resets when the counter reaches this channel's compare value and is set again at the top
/// of the period, which is exactly a duty cycle.
const OUTMOD_RESET_SET: u16 = 7 << OUTMOD_SHIFT;
/// Output simply follows the `OUT` bit. Used for the 0% and 100% ends, where a compare value would
/// otherwise produce a one-tick glitch.
const OUTMOD_CONSTANT: u16 = 0;

// Register offsets from the timer base.
const CTL: u16 = 0x00;
const CCR0: u16 = 0x12;
const EX0: u16 = 0x20;

/// Largest value `TBxEX0` can divide by.
const MAX_EX: u32 = 8;

/// PWM configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// Clock the timer counts.
    pub clock_source: PeripheralClock,
}

/// Reasons a frequency cannot be produced.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// The dividers cannot stretch or shrink the clock this far. Too low a frequency overflows the
    /// 16-bit counter even at the largest divider; too high a one leaves fewer than two counts per
    /// period, which is not a duty cycle any more. A timer without the expansion divider runs out
    /// of range eight times sooner.
    UnachievableFrequency,
    /// [`crate::init`] has not run, so the clock frequencies are unknown.
    ClocksNotInitialized,
}

trait SealedInstance {
    /// Address of the control register.
    const BASE: u16;
    /// Whether the timer has the expansion divider, `TxIDEX`. Timer_B has it; the F1xx Timer_A
    /// does not, which limits how far its period can be stretched.
    const HAS_IDEX: bool;
}

/// A Timer_B block usable for PWM.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static {}

pub(crate) trait SealedPwmPin<T> {
    /// Compare channel this pin is the output of, counting from 1.
    const CHANNEL: u8;
    /// Which of the pin's alternate functions this timer output is.
    const ALT: PinFunction;
}

/// A pin that is a timer's compare output.
#[allow(private_bounds)]
pub trait PwmPin<T: Instance>: SealedPwmPin<T> + Pin + PeripheralType {}

macro_rules! impl_instance {
    ($(#[$cfg:meta])? $peri:ident, $base:expr, $has_idex:expr) => {
        $(#[$cfg])?
        impl SealedInstance for peripherals::$peri {
            const BASE: u16 = $base;
            const HAS_IDEX: bool = $has_idex;
        }
        $(#[$cfg])?
        impl Instance for peripherals::$peri {}
    };
}

macro_rules! impl_pin {
    ($(#[$cfg:meta])? $timer:ident, $pin:ident, $channel:expr, $alt:expr) => {
        $(#[$cfg])?
        impl SealedPwmPin<peripherals::$timer> for peripherals::$pin {
            const CHANNEL: u8 = $channel;
            const ALT: PinFunction = $alt;
        }
        $(#[$cfg])?
        impl PwmPin<peripherals::$timer> for peripherals::$pin {}
    };
}

// The `embassy-time` driver's timer is only a peripheral of its own when it is not being used as
// the driver.
#[cfg(feature = "msp430fr2355")]
mod instances {
    use super::*;

    impl_instance!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        0x0380,
        true
    );
    impl_instance!(TB1, 0x03c0, true);
    impl_instance!(TB2, 0x0400, true);
    impl_instance!(TB3, 0x0440, true);

    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P1_6,
        1,
        PinFunction::Alternate2
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P1_7,
        2,
        PinFunction::Alternate2
    );
    impl_pin!(TB1, P2_0, 1, PinFunction::Alternate1);
    impl_pin!(TB1, P2_1, 2, PinFunction::Alternate1);
    impl_pin!(TB2, P5_0, 1, PinFunction::Alternate1);
    impl_pin!(TB2, P5_1, 2, PinFunction::Alternate1);
    impl_pin!(TB3, P6_0, 1, PinFunction::Alternate1);
    impl_pin!(TB3, P6_1, 2, PinFunction::Alternate1);
    impl_pin!(TB3, P6_2, 3, PinFunction::Alternate1);
    impl_pin!(TB3, P6_3, 4, PinFunction::Alternate1);
    impl_pin!(TB3, P6_4, 5, PinFunction::Alternate1);
    impl_pin!(TB3, P6_5, 6, PinFunction::Alternate1);
}

#[cfg(feature = "msp430f149")]
mod instances {
    use super::*;

    // Timer_B7's outputs are on P4; Timer_A3's are on P1. CCR0 sets the period in up mode, so the
    // channel that would drive P4.0 or P1.1 is not a PWM output.
    impl_instance!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        0x0180,
        true
    );
    impl_instance!(TA0, 0x0160, false);

    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P4_1,
        1,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P4_2,
        2,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P4_3,
        3,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P4_4,
        4,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P4_5,
        5,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P4_6,
        6,
        PinFunction::Alternate1
    );
    impl_pin!(TA0, P1_2, 1, PinFunction::Alternate1);
    impl_pin!(TA0, P1_3, 2, PinFunction::Alternate1);
}

#[inline]
fn reg(base: u16, offset: u16) -> *mut u16 {
    (base + offset) as *mut u16
}

#[inline]
fn write(base: u16, offset: u16, value: u16) {
    // SAFETY: a volatile write to a register of an existing Timer_B block.
    unsafe { reg(base, offset).write_volatile(value) }
}

/// Pick the smallest divider that fits `freq` into the 16-bit counter.
///
/// Smaller is better: the period in counts is the duty resolution, so dividing less leaves finer
/// steps. Returns `(ID exponent, TBIDEX, period in counts)`.
fn find_divider(clock: u32, freq: u32, has_idex: bool) -> Option<(u16, u32, u32)> {
    let max_ex = if has_idex { MAX_EX } else { 1 };
    for id_pow in 0..4u16 {
        for ex in 1..=max_ex {
            let div = (1u32 << id_pow) * ex;
            let period = clock / (div * freq);
            if (2..=65536).contains(&period) {
                return Some((id_pow, ex, period));
            }
        }
    }
    None
}

/// A timer set up to generate PWM.
pub struct Pwm<'d, T: Instance> {
    _timer: Peri<'d, T>,
    /// Counts in a period, which is also the value that means 100% duty.
    max_duty: u16,
}

impl<'d, T: Instance> Pwm<'d, T> {
    /// Set the timer up to run at `frequency` Hz.
    pub fn new(timer: Peri<'d, T>, frequency: u32, config: Config) -> Result<Self, ConfigError> {
        if frequency == 0 {
            return Err(ConfigError::UnachievableFrequency);
        }

        let clocks = crate::clocks().ok_or(ConfigError::ClocksNotInitialized)?;
        let (source_bits, source_hz) = match config.clock_source {
            PeripheralClock::Smclk => (TBSSEL_SMCLK, clocks.smclk),
            PeripheralClock::Aclk => (TBSSEL_ACLK, clocks.aclk),
        };

        let (id_pow, ex, period) =
            find_divider(source_hz, frequency, T::HAS_IDEX).ok_or(ConfigError::UnachievableFrequency)?;

        // Stop and clear before reprogramming, so the first period is a whole one.
        write(T::BASE, CTL, MC_STOP | TBCLR);
        if T::HAS_IDEX {
            write(T::BASE, EX0, (ex - 1) as u16);
        }
        // In up mode the counter runs 0..=CCR0, so a period of `period` counts needs CCR0 one less.
        write(T::BASE, CCR0, (period - 1) as u16);
        write(T::BASE, CTL, source_bits | (id_pow << ID_SHIFT) | MC_UP | TBCLR);

        Ok(Self {
            _timer: timer,
            max_duty: period as u16,
        })
    }

    /// Duty value that means 100%.
    pub fn max_duty_cycle(&self) -> u16 {
        self.max_duty
    }

    /// Attach `pin` as this timer's output for the channel it belongs to.
    ///
    /// The channel starts at 0% duty, so the pin is driven low until you say otherwise.
    pub fn channel<P: PwmPin<T>>(&mut self, pin: Peri<'d, P>) -> PwmChannel<'d> {
        let channel = P::CHANNEL as u16;
        let pin: Peri<'d, AnyPin> = pin.into();

        let mut ch = PwmChannel {
            cctl: reg(T::BASE, 0x02 + channel * 2),
            ccr: reg(T::BASE, CCR0 + channel * 2),
            max_duty: self.max_duty,
            pin,
            _lifetime: PhantomData,
        };
        ch.set_duty(0);

        gpio::set_alternate(&ch.pin, P::ALT);
        ch
    }
}

impl<'d, T: Instance> Drop for Pwm<'d, T> {
    fn drop(&mut self) {
        write(T::BASE, CTL, MC_STOP | TBCLR);
    }
}

/// One PWM output.
pub struct PwmChannel<'d> {
    cctl: *mut u16,
    ccr: *mut u16,
    max_duty: u16,
    pin: Peri<'d, AnyPin>,
    _lifetime: PhantomData<&'d ()>,
}

impl<'d> PwmChannel<'d> {
    /// Duty value that means 100%.
    pub fn max_duty_cycle(&self) -> u16 {
        self.max_duty
    }

    /// Set the duty cycle, from 0 to [`max_duty_cycle`](Self::max_duty_cycle).
    ///
    /// Values above the maximum are treated as 100%.
    pub fn set_duty(&mut self, duty: u16) {
        // SAFETY: `cctl` and `ccr` are this channel's own registers; no other channel or the
        // `Pwm` itself touches them, so no synchronisation is needed.
        unsafe {
            if duty == 0 {
                // A compare value of 0 would still produce a one-tick pulse, so drive the pin
                // from the OUT bit instead of from the comparator.
                self.cctl.write_volatile(OUTMOD_CONSTANT);
            } else if duty >= self.max_duty {
                self.cctl.write_volatile(OUTMOD_CONSTANT | OUT);
            } else {
                self.ccr.write_volatile(duty);
                self.cctl.write_volatile(OUTMOD_RESET_SET);
            }
        }
    }

    /// Set the duty cycle as a percentage.
    pub fn set_duty_percent(&mut self, percent: u8) {
        let percent = percent.min(100) as u32;
        let duty = (self.max_duty as u32 * percent) / 100;
        self.set_duty(duty as u16);
    }
}

impl<'d> Drop for PwmChannel<'d> {
    fn drop(&mut self) {
        self.set_duty(0);
        gpio::set_gpio_function(&self.pin);
    }
}

impl embedded_hal::pwm::ErrorType for PwmChannel<'_> {
    type Error = Infallible;
}

impl embedded_hal::pwm::SetDutyCycle for PwmChannel<'_> {
    fn max_duty_cycle(&self) -> u16 {
        self.max_duty
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.set_duty(duty);
        Ok(())
    }
}

/// Timer_A on the MSP430FR6043, and which pins its compare outputs reach.
///
/// Timer_B0 is the `embassy-time` driver here, so PWM comes off the five Timer_A instances instead.
/// They have the same register layout — including the `TAxEX0` divider — so the same driver runs
/// both.
///
/// The alternate function each output sits on is derived from the order of the functions in the
/// datasheet's package pinout, quoted beside each line. See the note in `uart::TxPin`: these want
/// checking against Table 7-1 before they are trusted on hardware.
#[cfg(feature = "_fr504x_604x")]
mod instances_fr6043 {
    use super::*;

    impl_instance!(TA0, 0x0340, true);
    impl_instance!(TA1, 0x0380, true);
    impl_instance!(TA2, 0x0400, true);
    impl_instance!(TA3, 0x0440, true);
    impl_instance!(TA4, 0x07c0, true);
    impl_instance!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        0x03c0,
        true
    );

    // P2.3/TA0.0/UCA0STE, P5.4/TA0.0/UCB1CLK/TA4.0. P2.3 is 80-pin only, so the FR5043 reaches
    // this channel through P5.4 alone -- which is why every channel below keeps a pin that both
    // packages bring out.
    impl_pin!(
        #[cfg(feature = "msp430fr6043")]
        TA0,
        P2_3,
        1,
        PinFunction::Alternate1
    );
    impl_pin!(TA0, P5_4, 1, PinFunction::Alternate1);
    // P2.5/TA0.2/TA4.0 (80-pin only), P5.7/TA0.2/UCB1STE
    impl_pin!(
        #[cfg(feature = "msp430fr6043")]
        TA0,
        P2_5,
        3,
        PinFunction::Alternate1
    );
    impl_pin!(TA0, P5_7, 3, PinFunction::Alternate1);

    // P1.0/UCA1CLK/TA1.0, P7.0/TA1.0/TA1.2
    impl_pin!(TA1, P1_0, 1, PinFunction::Alternate2);
    impl_pin!(TA1, P7_0, 1, PinFunction::Alternate1);
    // P1.3/UCA1SOMI/UCA1RXD/TA1.1
    impl_pin!(TA1, P1_3, 2, PinFunction::Alternate3);
    // P2.6/UCA0SIMO/UCA0TXD/TA1.2 -- 80-pin only, and the only pin for TA1.2, so this channel is
    // not reachable on the FR5043.
    impl_pin!(
        #[cfg(feature = "msp430fr6043")]
        TA1,
        P2_6,
        3,
        PinFunction::Alternate2
    );

    // P1.1/UCA1STE/TA4.0
    impl_pin!(TA4, P1_1, 1, PinFunction::Alternate2);
    // P5.5/TA4.1/UCB1SIMO/UCB1SDA, P4.0/RTCCLK/TA4.1
    impl_pin!(TA4, P5_5, 2, PinFunction::Alternate1);
    impl_pin!(TA4, P4_0, 2, PinFunction::Alternate2);

    // Timer_B0's own outputs, for a build that does not use it as the time driver.
    // P3.0/TB0.0, P5.1/TB0.1, P5.2/TB0.2, P5.3/TB0.3, P1.4/TB0.4, P1.5/TB0.5
    impl_pin!(
        #[cfg(all(not(feature = "time-driver-tb0"), feature = "msp430fr6043"))]
        TB0,
        P3_0,
        1,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P5_1,
        2,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P5_2,
        3,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P5_3,
        4,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P1_4,
        5,
        PinFunction::Alternate1
    );
    impl_pin!(
        #[cfg(not(feature = "time-driver-tb0"))]
        TB0,
        P1_5,
        6,
        PinFunction::Alternate1
    );
}
