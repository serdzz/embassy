//! Analog-to-digital converter.
//!
//! A single 12-bit SAR converter, shared by every channel, so [`Adc`] owns it and conversions are
//! taken one at a time.
//!
//! ```ignore
//! let mut adc = Adc::new(p.ADC, adc::Config::default());
//! let raw = adc.read(&mut p.P1_1).await;
//! ```
//!
//! # Internal channels
//!
//! Channels above A11 are internal — the temperature sensor and the supply monitor — and their
//! numbers differ between devices. [`Adc::read_channel`] takes a raw channel number for those;
//! look it up in your device's datasheet.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

use crate::gpio::{self, Pin};
use crate::{pac, peripherals};

const BASE: u16 = 0x0700;

// Control register 0.
const ADCSC: u16 = 0x0001;
const ADCENC: u16 = 0x0002;
const ADCON: u16 = 0x0010;
const ADCSHT_SHIFT: u16 = 8;

// Control register 1.
const ADCSSEL_SHIFT: u16 = 3;
const ADCDIV_SHIFT: u16 = 5;
const ADCSHP: u16 = 0x0200;

// Control register 2.
const ADCRES_SHIFT: u16 = 4;

// Conversion memory control.
const ADCSREF_SHIFT: u16 = 4;

// Interrupt enable and flags.
const ADCIE0: u16 = 0x0001;
const ADCIFG0: u16 = 0x0001;

// Register offsets from the peripheral base.
const CTL0: u16 = 0x00;
const CTL1: u16 = 0x02;
const CTL2: u16 = 0x04;
const MCTL0: u16 = 0x0a;
const MEM0: u16 = 0x12;
const IE: u16 = 0x1a;
const IFG: u16 = 0x1c;

static WAKER: AtomicWaker = AtomicWaker::new();

#[inline]
fn reg(offset: u16) -> *mut u16 {
    (BASE + offset) as *mut u16
}

#[inline]
fn read(offset: u16) -> u16 {
    // SAFETY: a volatile read of an ADC register.
    unsafe { reg(offset).read_volatile() }
}

#[inline]
fn write(offset: u16, value: u16) {
    // SAFETY: a volatile write to an ADC register.
    unsafe { reg(offset).write_volatile(value) }
}

#[inline]
fn modify(offset: u16, f: impl FnOnce(u16) -> u16) {
    critical_section::with(|_| write(offset, f(read(offset))));
}

/// Conversion resolution.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resolution {
    /// 8 bits. The fastest conversion.
    _8Bit,
    /// 10 bits.
    _10Bit,
    /// 12 bits.
    #[default]
    _12Bit,
}

impl Resolution {
    const fn bits(self) -> u16 {
        self as u16
    }

    /// Largest value a conversion can produce.
    pub const fn max_value(self) -> u16 {
        match self {
            Resolution::_8Bit => 0xFF,
            Resolution::_10Bit => 0x3FF,
            Resolution::_12Bit => 0xFFF,
        }
    }
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
    /// Internal 2.0 V reference. Needs a supply above about 2.2 V.
    Internal2V0,
    /// Internal 2.5 V reference. Needs a supply above about 2.7 V.
    Internal2V5,
}

impl Reference {
    /// `ADCSREF` field.
    const fn sref(self) -> u16 {
        match self {
            Reference::Vcc => 0,
            _ => 1,
        }
    }

    /// `REFVSEL` field, for the references the PMM generates.
    const fn refvsel(self) -> Option<u8> {
        match self {
            Reference::Vcc => None,
            Reference::Internal1V5 => Some(0),
            Reference::Internal2V0 => Some(1),
            Reference::Internal2V5 => Some(2),
        }
    }

    /// Nominal full-scale voltage in millivolts.
    pub const fn millivolts(self) -> Option<u16> {
        match self {
            Reference::Vcc => None,
            Reference::Internal1V5 => Some(1500),
            Reference::Internal2V0 => Some(2000),
            Reference::Internal2V5 => Some(2500),
        }
    }
}

/// How long the input is sampled, in ADC clocks.
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
    /// The dedicated MODCLK, about 5 MHz. Independent of how the rest of the system is clocked,
    /// and it keeps the converter usable when MCLK is slow.
    #[default]
    Modclk,
    /// ACLK.
    Aclk,
    /// SMCLK.
    Smclk,
}

impl AdcClock {
    const fn bits(self) -> u16 {
        match self {
            AdcClock::Modclk => 0,
            AdcClock::Aclk => 1,
            AdcClock::Smclk => 2,
        }
    }
}

/// Divider applied to the ADC clock.
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

/// ADC configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// Conversion resolution.
    pub resolution: Resolution,
    /// Voltage the conversion is measured against.
    pub reference: Reference,
    /// How long the input is sampled.
    pub sample_time: SampleTime,
    /// Clock the converter runs from.
    pub clock: AdcClock,
    /// Divider applied to that clock.
    pub clock_div: ClockDiv,
}

pub(crate) trait SealedAdcChannel {
    fn channel(&self) -> u8;
}

/// A pin that is wired to an ADC channel.
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

impl_channel!(P1_0, 0);
impl_channel!(P1_1, 1);
impl_channel!(P1_2, 2);
impl_channel!(P1_3, 3);
impl_channel!(P1_4, 4);
impl_channel!(P1_5, 5);
impl_channel!(P1_6, 6);
impl_channel!(P1_7, 7);
impl_channel!(P5_0, 8);
impl_channel!(P5_1, 9);
impl_channel!(P5_2, 10);
impl_channel!(P5_3, 11);

/// The analog-to-digital converter.
pub struct Adc<'d> {
    _peri: Peri<'d, peripherals::ADC>,
    resolution: Resolution,
    reference: Reference,
}

impl<'d> Adc<'d> {
    /// Turn the converter on and apply `config`.
    ///
    /// If `config` asks for an internal reference, this waits for the reference generator to
    /// settle, which takes a few tens of microseconds.
    pub fn new(peri: Peri<'d, peripherals::ADC>, config: Config) -> Self {
        if let Some(refvsel) = config.reference.refvsel() {
            let pmm = unsafe { pac::Pmm::steal() };
            pmm.pmmctl2().modify(|_, w| unsafe {
                w.intrefen().set_bit();
                w.refvsel().bits(refvsel)
            });
            // The generator needs a moment before a conversion against it means anything.
            while pmm.pmmctl2().read().refgenrdy().bit_is_clear() {}
        }

        // ADCENC has to be clear while the control registers are written.
        write(CTL0, 0);
        write(
            CTL1,
            ADCSHP | (config.clock.bits() << ADCSSEL_SHIFT) | (config.clock_div.bits() << ADCDIV_SHIFT),
        );
        write(CTL2, config.resolution.bits() << ADCRES_SHIFT);
        write(CTL0, (config.sample_time.bits() << ADCSHT_SHIFT) | ADCON);
        write(IE, 0);
        write(IFG, 0);

        Self {
            _peri: peri,
            resolution: config.resolution,
            reference: config.reference,
        }
    }

    /// Resolution the converter was configured for.
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// Reference the converter was configured for.
    pub fn reference(&self) -> Reference {
        self.reference
    }

    /// Convert a raw reading to millivolts, if the reference has a known voltage.
    ///
    /// Returns `None` for [`Reference::Vcc`], whose voltage the chip has no way of knowing.
    pub fn to_millivolts(&self, raw: u16) -> Option<u16> {
        let full_scale = self.reference.millivolts()?;
        let max = self.resolution.max_value() as u32;
        Some(((raw as u32 * full_scale as u32) / max) as u16)
    }

    /// Point the converter at `channel` and start a conversion.
    fn start(&mut self, channel: u8) {
        // MCTL0 can only be written while ADCENC is clear.
        modify(CTL0, |v| v & !ADCENC);
        write(
            MCTL0,
            (channel as u16 & 0x0f) | (self.reference.sref() << ADCSREF_SHIFT),
        );
        write(IFG, 0);
        modify(CTL0, |v| v | ADCENC | ADCSC);
    }

    /// Take one conversion from `channel`, spinning.
    ///
    /// The channel number is the raw `ADCINCH` value, which lets you reach the internal channels
    /// that have no pin.
    pub fn blocking_read_channel(&mut self, channel: u8) -> u16 {
        self.start(channel);
        while read(IFG) & ADCIFG0 == 0 {}
        // Reading the result clears the flag.
        read(MEM0)
    }

    /// Take one conversion from `channel`, suspending until it finishes.
    pub async fn read_channel(&mut self, channel: u8) -> u16 {
        self.start(channel);

        poll_fn(|cx| {
            critical_section::with(|_| {
                if read(IFG) & ADCIFG0 != 0 {
                    Poll::Ready(())
                } else {
                    // Register before unmasking: the handler cannot run until this critical
                    // section ends, so the wakeup cannot be missed.
                    WAKER.register(cx.waker());
                    modify(IE, |v| v | ADCIE0);
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

impl<'d> Drop for Adc<'d> {
    fn drop(&mut self) {
        modify(IE, |v| v & !ADCIE0);
        // Clearing ADCON powers the analog block down; it is the bulk of what the ADC costs.
        modify(CTL0, |v| v & !(ADCENC | ADCON));

        if self.reference.refvsel().is_some() {
            let pmm = unsafe { pac::Pmm::steal() };
            pmm.pmmctl2().modify(|_, w| w.intrefen().clear_bit());
        }
    }
}

embassy_executor::msp430_interrupt! {
    /// Conversion complete.
    unsafe fn ADC() {
        // The flag is cleared by reading MEM0, which is the task's job. Masking instead both
        // silences the interrupt and leaves the flag as the "is it done" answer.
        modify(IE, |v| v & !ADCIE0);
        WAKER.wake();
    }
}
