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
use crate::gpio::{self, AnyPin, Pin};
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
    /// period, which is not a duty cycle any more.
    UnachievableFrequency,
    /// [`crate::init`] has not run, so the clock frequencies are unknown.
    ClocksNotInitialized,
}

trait SealedInstance {
    const BASE: u16;
}

/// A Timer_B block usable for PWM.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static {}

/// Which alternate function a timer's outputs live on.
#[derive(Copy, Clone)]
pub(crate) enum Alt {
    One,
    /// Only TB0's outputs live here, and TB0 is normally the `embassy-time` driver, so with the
    /// default features nothing constructs this.
    #[allow(dead_code)]
    Two,
}

pub(crate) trait SealedPwmPin<T> {
    /// Compare channel this pin is the output of, counting from 1.
    const CHANNEL: u8;
    const ALT: Alt;
}

/// A pin that is a timer's compare output.
#[allow(private_bounds)]
pub trait PwmPin<T: Instance>: SealedPwmPin<T> + Pin + PeripheralType {}

macro_rules! impl_instance {
    ($(#[$cfg:meta])? $peri:ident, $base:expr) => {
        $(#[$cfg])?
        impl SealedInstance for peripherals::$peri {
            const BASE: u16 = $base;
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
            const ALT: Alt = $alt;
        }
        $(#[$cfg])?
        impl PwmPin<peripherals::$timer> for peripherals::$pin {}
    };
}

// TB0 is only a peripheral of its own when it is not the `embassy-time` driver.
impl_instance!(
    #[cfg(not(feature = "time-driver-tb0"))]
    TB0,
    0x0380
);
impl_instance!(TB1, 0x03c0);
impl_instance!(TB2, 0x0400);
impl_instance!(TB3, 0x0440);

impl_pin!(
    #[cfg(not(feature = "time-driver-tb0"))]
    TB0,
    P1_6,
    1,
    Alt::Two
);
impl_pin!(
    #[cfg(not(feature = "time-driver-tb0"))]
    TB0,
    P1_7,
    2,
    Alt::Two
);
impl_pin!(TB1, P2_0, 1, Alt::One);
impl_pin!(TB1, P2_1, 2, Alt::One);
impl_pin!(TB2, P5_0, 1, Alt::One);
impl_pin!(TB2, P5_1, 2, Alt::One);
impl_pin!(TB3, P6_0, 1, Alt::One);
impl_pin!(TB3, P6_1, 2, Alt::One);
impl_pin!(TB3, P6_2, 3, Alt::One);
impl_pin!(TB3, P6_3, 4, Alt::One);
impl_pin!(TB3, P6_4, 5, Alt::One);
impl_pin!(TB3, P6_5, 6, Alt::One);

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
fn find_divider(clock: u32, freq: u32) -> Option<(u16, u32, u32)> {
    for id_pow in 0..4u16 {
        for ex in 1..=MAX_EX {
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

        let (id_pow, ex, period) = find_divider(source_hz, frequency).ok_or(ConfigError::UnachievableFrequency)?;

        // Stop and clear before reprogramming, so the first period is a whole one.
        write(T::BASE, CTL, MC_STOP | TBCLR);
        write(T::BASE, EX0, (ex - 1) as u16);
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

        match P::ALT {
            Alt::One => gpio::set_alternate1(&ch.pin),
            Alt::Two => gpio::set_alternate2(&ch.pin),
        }
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
