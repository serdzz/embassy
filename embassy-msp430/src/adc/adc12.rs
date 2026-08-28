//! The F1xx converter: the ADC12.
//!
//! Twelve bits, sixteen channels, one conversion at a time here.
//!
//! ```ignore
//! let mut adc = Adc::new(p.ADC12, adc::Config::default());
//! let raw = adc.read(&mut p.P6_1).await;
//! ```
//!
//! # Internal channels
//!
//! Above A7 the channels are internal rather than pins: the external reference inputs, the
//! temperature sensor and half the supply. [`Channel`] names them, and [`Adc::read_channel`] takes
//! a raw number for anything it does not.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

use crate::gpio::{self, Pin};
use crate::peripherals;

const BASE: u16 = 0x0080;

// Register offsets from the base. The memory control registers are bytes; the rest are words.
const MCTL0: u16 = 0x00;
const MEM0: u16 = 0xc0;
const CTL0: u16 = 0x120;
const CTL1: u16 = 0x122;
const IFG: u16 = 0x124;
const IE: u16 = 0x126;

// Control register 0.
const ADC12SC: u16 = 0x0001;
const ENC: u16 = 0x0002;
const ADC12ON: u16 = 0x0010;
const REFON: u16 = 0x0020;
const REF2_5V: u16 = 0x0040;
const SHT0_SHIFT: u16 = 8;

// Control register 1.
const ADC12SSEL_SHIFT: u16 = 3;
const ADC12DIV_SHIFT: u16 = 5;
const SHP: u16 = 0x0200;

// Memory control register.
const SREF_SHIFT: u8 = 4;

// Interrupt flag and enable, for conversion memory 0.
const IFG0: u16 = 0x0001;
const IE0: u16 = 0x0001;

static WAKER: AtomicWaker = AtomicWaker::new();

#[inline]
fn read(offset: u16) -> u16 {
    // SAFETY: a volatile read of an ADC12 register.
    unsafe { ((BASE + offset) as *mut u16).read_volatile() }
}

#[inline]
fn write(offset: u16, value: u16) {
    // SAFETY: a volatile write to an ADC12 register.
    unsafe { ((BASE + offset) as *mut u16).write_volatile(value) }
}

#[inline]
fn modify(offset: u16, f: impl FnOnce(u16) -> u16) {
    critical_section::with(|_| write(offset, f(read(offset))));
}

#[inline]
fn write_mctl(value: u8) {
    // SAFETY: a volatile write to the first conversion memory control register, which is a byte.
    unsafe { ((BASE + MCTL0) as *mut u8).write_volatile(value) }
}

/// Voltage the conversion is measured against.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reference {
    /// The supply rail. Needs no settling time, but is only as accurate as the supply.
    #[default]
    Vcc,
    /// Internal 1.5 V reference.
    Internal1V5,
    /// Internal 2.5 V reference. Needs a supply above about 2.9 V.
    Internal2V5,
}

impl Reference {
    /// `SREF` field: 0 measures against the supply, 1 against the reference generator.
    const fn sref(self) -> u8 {
        match self {
            Reference::Vcc => 0,
            _ => 1,
        }
    }

    /// Nominal full-scale voltage in millivolts, where the chip knows it.
    pub const fn millivolts(self) -> Option<u16> {
        match self {
            Reference::Vcc => None,
            Reference::Internal1V5 => Some(1500),
            Reference::Internal2V5 => Some(2500),
        }
    }
}

/// How long the input is sampled, in converter clocks.
///
/// The source's impedance decides how much is enough: the sampling capacitor has to charge through
/// it. Longer is always safe, just slower.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(non_camel_case_types)]
pub enum SampleTime {
    /// 4 clocks.
    _4,
    /// 8 clocks. Fine for a low-impedance source.
    #[default]
    _8,
    /// 16 clocks.
    _16,
    /// 32 clocks.
    _32,
    /// 64 clocks.
    _64,
    /// 96 clocks.
    _96,
    /// 128 clocks.
    _128,
    /// 192 clocks.
    _192,
    /// 256 clocks.
    _256,
}

impl SampleTime {
    const fn bits(self) -> u16 {
        self as u16
    }
}

/// Clock the converter runs from.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcClock {
    /// The converter's own oscillator, about 5 MHz. Independent of how the rest of the system is
    /// clocked, which on this family means independent of an untrimmed DCO.
    #[default]
    Internal,
    /// ACLK.
    Aclk,
    /// MCLK.
    Mclk,
    /// SMCLK.
    Smclk,
}

impl AdcClock {
    const fn bits(self) -> u16 {
        self as u16
    }
}

/// Divider applied to the converter clock.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockDiv {
    /// Divide by 1.
    #[default]
    _1,
    /// Divide by 2.
    _2,
    /// Divide by 3.
    _3,
    /// Divide by 4.
    _4,
    /// Divide by 5.
    _5,
    /// Divide by 6.
    _6,
    /// Divide by 7.
    _7,
    /// Divide by 8.
    _8,
}

impl ClockDiv {
    const fn bits(self) -> u16 {
        self as u16
    }
}

/// ADC12 configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// Voltage the conversion is measured against.
    pub reference: Reference,
    /// How long the input is sampled.
    pub sample_time: SampleTime,
    /// Clock the converter runs from.
    pub clock: AdcClock,
    /// Divider applied to that clock.
    pub clock_div: ClockDiv,
}

/// The channels that are not pins.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Channel {
    /// The external positive reference input.
    VeRefPlus,
    /// The external negative reference input.
    VeRefMinus,
    /// The on-chip temperature sensor.
    Temperature,
    /// Half the supply voltage, for monitoring a battery.
    HalfVcc,
}

impl Channel {
    /// Raw `INCH` value.
    pub const fn channel(self) -> u8 {
        match self {
            Channel::VeRefPlus => 8,
            Channel::VeRefMinus => 9,
            Channel::Temperature => 10,
            Channel::HalfVcc => 11,
        }
    }
}

pub(crate) trait SealedAdcChannel {
    fn channel(&self) -> u8;
}

/// A pin that is wired to a converter channel.
#[allow(private_bounds)]
pub trait AdcChannel: SealedAdcChannel + Pin + PeripheralType {}

macro_rules! impl_channel {
    ($pin:ident, $channel:expr) => {
        impl SealedAdcChannel for peripherals::$pin {
            fn channel(&self) -> u8 {
                $channel
            }
        }
        impl AdcChannel for peripherals::$pin {}
    };
}

// A0 through A7 are P6.0 through P6.7.
impl_channel!(P6_0, 0);
impl_channel!(P6_1, 1);
impl_channel!(P6_2, 2);
impl_channel!(P6_3, 3);
impl_channel!(P6_4, 4);
impl_channel!(P6_5, 5);
impl_channel!(P6_6, 6);
impl_channel!(P6_7, 7);

/// Largest value a conversion can produce.
pub const MAX_VALUE: u16 = 0x0FFF;

/// The analog-to-digital converter.
pub struct Adc<'d> {
    _peri: Peri<'d, peripherals::ADC12>,
    reference: Reference,
}

impl<'d> Adc<'d> {
    /// Turn the converter on and apply `config`.
    ///
    /// If `config` asks for the internal reference, this waits for the generator to settle. There
    /// is no ready flag for that on this converter, so the wait is a timed one, worked out from
    /// MCLK — which on this family is an untrimmed DCO, so give it room by telling
    /// [`crate::clock`] what the DCO really runs at.
    pub fn new(peri: Peri<'d, peripherals::ADC12>, config: Config) -> Self {
        // ENC has to be clear while the control registers are written.
        write(CTL0, 0);
        write(
            CTL1,
            SHP | (config.clock.bits() << ADC12SSEL_SHIFT) | (config.clock_div.bits() << ADC12DIV_SHIFT),
        );

        let mut ctl0 = (config.sample_time.bits() << SHT0_SHIFT) | ADC12ON;
        match config.reference {
            Reference::Vcc => {}
            Reference::Internal1V5 => ctl0 |= REFON,
            Reference::Internal2V5 => ctl0 |= REFON | REF2_5V,
        }
        write(CTL0, ctl0);
        write(IE, 0);
        write(IFG, 0);

        if config.reference != Reference::Vcc {
            settle_reference();
        }

        Self {
            _peri: peri,
            reference: config.reference,
        }
    }

    /// Reference the converter was configured for.
    pub fn reference(&self) -> Reference {
        self.reference
    }

    /// Convert a raw reading to millivolts, if the reference has a known voltage.
    ///
    /// Returns `None` for [`Reference::Vcc`], whose voltage the chip has no way of knowing.
    pub fn to_millivolts(&self, raw: u16) -> Option<u16> {
        let full_scale = self.reference.millivolts()? as u32;
        Some(((raw as u32 * full_scale) / MAX_VALUE as u32) as u16)
    }

    /// Point the converter at `channel` and start a conversion.
    fn start(&mut self, channel: u8) {
        // The conversion memory control register can only be written while ENC is clear.
        modify(CTL0, |v| v & !ENC);
        write_mctl((channel & 0x0f) | (self.reference.sref() << SREF_SHIFT));
        write(IFG, 0);
        modify(CTL0, |v| v | ENC | ADC12SC);
    }

    /// Take one conversion from `channel`, spinning.
    ///
    /// The channel number is the raw `INCH` value; [`Channel`] names the ones that are not pins.
    pub fn blocking_read_channel(&mut self, channel: u8) -> u16 {
        self.start(channel);
        while read(IFG) & IFG0 == 0 {}
        // Reading the result clears the flag.
        read(MEM0)
    }

    /// Take one conversion from `channel`, suspending until it finishes.
    pub async fn read_channel(&mut self, channel: u8) -> u16 {
        self.start(channel);

        poll_fn(|cx| {
            critical_section::with(|_| {
                if read(IFG) & IFG0 != 0 {
                    Poll::Ready(())
                } else {
                    // Register before unmasking: the handler cannot run until this critical
                    // section ends, so the wakeup cannot be missed.
                    WAKER.register(cx.waker());
                    modify(IE, |v| v | IE0);
                    Poll::Pending
                }
            })
        })
        .await;

        read(MEM0)
    }

    /// Take one conversion from `pin`, spinning.
    pub fn blocking_read<P: AdcChannel>(&mut self, pin: &mut Peri<'_, P>) -> u16 {
        let channel = pin.channel();
        gpio::set_analog(&**pin);
        self.blocking_read_channel(channel)
    }

    /// Take one conversion from `pin`, suspending until it finishes.
    pub async fn read<P: AdcChannel>(&mut self, pin: &mut Peri<'_, P>) -> u16 {
        let channel = pin.channel();
        gpio::set_analog(&**pin);
        self.read_channel(channel).await
    }
}

/// Give the reference generator its settling time, about 20 µs.
///
/// The loop is four instructions or so per iteration; the exact figure does not matter as long as
/// it errs long, and it does.
fn settle_reference() {
    let mclk = crate::clocks().map_or(1_000_000, |c| c.mclk);
    let iterations = (mclk / 50_000).max(1);
    for _ in 0..iterations {
        msp430::asm::nop();
    }
}

impl<'d> Drop for Adc<'d> {
    fn drop(&mut self) {
        modify(IE, |v| v & !IE0);
        // Clearing ADC12ON and REFON powers the analog blocks down; between them they are the bulk
        // of what the converter costs.
        modify(CTL0, |v| v & !(ENC | ADC12ON | REFON));
    }
}

embassy_executor::msp430_interrupt! {
    /// Conversion complete.
    unsafe fn ADC12() {
        // The flag is cleared by reading MEM0, which is the task's job. Masking instead both
        // silences the interrupt and leaves the flag as the "is it done" answer.
        modify(IE, |v| v & !IE0);
        WAKER.wake();
    }
}
