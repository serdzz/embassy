//! ADC12_B: a 12-bit successive-approximation converter with 32 conversion-memory slots.
//!
//! This driver uses one of those slots. The other 31 exist for sequence and repeat modes, which are
//! how the converter earns its keep in a meter sampling several channels back to back — but they
//! need the DMA to be worth having, and there is no DMA driver here yet. A single conversion at a
//! time is what this offers.
//!
//! # Where the reference comes from
//!
//! Two choices. `AVCC` costs nothing and is worth exactly what the supply happens to be, which on a
//! battery is not a known number. The internal reference is a separate module — `REF_A` — that has
//! to be switched on and given time to settle, and then it is 1.2, 2.0 or 2.5 volts to within a
//! percent. [`Adc::new`] does that settling, so a conversion started immediately afterwards is
//! already valid.

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

use crate::gpio::SealedPin;
use crate::peripherals;

const BASE: u16 = 0x0800;

const CTL0: u16 = 0x00;
const CTL1: u16 = 0x02;
const CTL2: u16 = 0x04;
const CTL3: u16 = 0x06;
const IFGR0: u16 = 0x0c;
const IER0: u16 = 0x12;
const MCTL0: u16 = 0x20;
const MEM0: u16 = 0x60;

// ADC12CTL0.
const ADC12SC: u16 = 1 << 0;
const ADC12ENC: u16 = 1 << 1;
const ADC12ON: u16 = 1 << 4;

// ADC12CTL1.
const ADC12SHP: u16 = 1 << 9;

/// `REF_A`, the internal reference.
const REFCTL0: u16 = 0x01b0;
const REFON: u16 = 1 << 0;
const REFGENRDY: u16 = 1 << 11;

/// The one waker: this driver runs one conversion at a time.
static WAKER: AtomicWaker = AtomicWaker::new();

/// Where the converter's full scale comes from.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reference {
    /// The supply rail. Free, and only as accurate as the supply.
    #[default]
    Avcc,
    /// 1.2 V from the internal reference.
    Internal1V2,
    /// 2.0 V from the internal reference.
    Internal2V0,
    /// 2.5 V from the internal reference. Needs a supply above about 2.7 V.
    Internal2V5,
}

impl Reference {
    /// `ADC12VRSEL` value: 0 is AVCC, 1 is the internal reference against AVSS.
    const fn vrsel(self) -> u16 {
        match self {
            Reference::Avcc => 0,
            _ => 1,
        }
    }

    /// `REFVSEL` value, for the references that need `REF_A`.
    const fn refvsel(self) -> Option<u16> {
        match self {
            Reference::Avcc => None,
            Reference::Internal1V2 => Some(0),
            Reference::Internal2V0 => Some(1),
            Reference::Internal2V5 => Some(2),
        }
    }

    /// Full scale in millivolts, where it is a known number.
    ///
    /// `None` for `AVCC`, because the supply is whatever it is: converting a reading to millivolts
    /// needs a number this driver has no way to know.
    pub const fn millivolts(self) -> Option<u16> {
        match self {
            Reference::Avcc => None,
            Reference::Internal1V2 => Some(1200),
            Reference::Internal2V0 => Some(2000),
            Reference::Internal2V5 => Some(2500),
        }
    }
}

/// How long the input is connected to the sampling capacitor, in ADC clocks.
///
/// The capacitor has to charge through whatever impedance the source has. Too short and the reading
/// is low; the datasheet's rule of thumb is that a source above a few kiloohms needs the longer
/// settings.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum SampleTime {
    _4 = 0,
    _8 = 1,
    _16 = 2,
    _32 = 3,
    #[default]
    _64 = 4,
    _96 = 5,
    _128 = 6,
    _192 = 7,
    _256 = 8,
    _384 = 9,
    _512 = 10,
    _768 = 11,
    _1024 = 12,
}

/// Resolution, which also sets how long a conversion takes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resolution {
    /// 8 bits, 10 clocks per conversion.
    _8Bit = 0,
    /// 10 bits, 12 clocks.
    _10Bit = 1,
    /// 12 bits, 14 clocks.
    #[default]
    _12Bit = 2,
}

impl Resolution {
    /// The largest reading this resolution produces.
    pub const fn max_value(self) -> u16 {
        match self {
            Resolution::_8Bit => 0x00ff,
            Resolution::_10Bit => 0x03ff,
            Resolution::_12Bit => 0x0fff,
        }
    }
}

/// Clock the converter runs from.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcClock {
    /// The converter's own oscillator, around 5 MHz. Runs whatever else is asleep.
    #[default]
    Modclk,
    /// ACLK.
    Aclk,
    /// SMCLK.
    Smclk,
}

impl AdcClock {
    /// `ADC12SSEL` value.
    const fn bits(self) -> u16 {
        match self {
            AdcClock::Modclk => 0,
            AdcClock::Aclk => 1,
            AdcClock::Smclk => 3,
        }
    }
}

/// Converter configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// Where full scale comes from.
    pub reference: Reference,
    /// How long each input is sampled for.
    pub sample_time: SampleTime,
    /// Resolution.
    pub resolution: Resolution,
    /// Clock source.
    pub clock: AdcClock,
}

/// The internal channels, which are not pins.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Channel {
    /// The on-chip temperature sensor.
    ///
    /// Reading it needs the internal reference: against `AVCC` the number moves with the supply and
    /// means nothing.
    TemperatureSensor,
    /// Half the supply rail, which is how the supply itself is measured.
    HalfAvcc,
}

impl Channel {
    /// `ADC12INCH` value.
    ///
    /// Both live on channels the `ADC12CTL3` map registers point at; these are the reset defaults,
    /// which this driver does not change.
    pub const fn channel(self) -> u8 {
        match self {
            Channel::TemperatureSensor => 30,
            Channel::HalfAvcc => 31,
        }
    }
}

/// A pin that can be converted.
#[allow(private_bounds)]
pub trait AdcChannel: SealedPin + PeripheralType {
    /// `ADC12INCH` value for this pin.
    const CHANNEL: u8;
}

macro_rules! impl_channel {
    ($pin:ident, $channel:expr) => {
        impl AdcChannel for peripherals::$pin {
            const CHANNEL: u8 = $channel;
        }
    };
}

// From the package pinout: A0 and A1 are on P1.0 and P1.1, A2 and A3 on P1.4 and P1.5, A8 and A9 on
// P1.2 and P1.3, A14 and A15 on P2.2 and P2.3. The analog function is the third alternate, which
// `set_analog` selects.
impl_channel!(P1_0, 0);
impl_channel!(P1_1, 1);
impl_channel!(P1_4, 2);
impl_channel!(P1_5, 3);
impl_channel!(P1_2, 8);
impl_channel!(P1_3, 9);
impl_channel!(P2_2, 14);
impl_channel!(P2_3, 15);

#[inline]
fn read(offset: u16) -> u16 {
    // SAFETY: a volatile read of an ADC12_B register.
    unsafe { ((BASE + offset) as *mut u16).read_volatile() }
}

#[inline]
fn write(offset: u16, value: u16) {
    // SAFETY: a volatile write to an ADC12_B register.
    unsafe { ((BASE + offset) as *mut u16).write_volatile(value) }
}

#[inline]
fn read_ref() -> u16 {
    // SAFETY: a volatile read of REFCTL0.
    unsafe { (REFCTL0 as *mut u16).read_volatile() }
}

#[inline]
fn write_ref(value: u16) {
    // SAFETY: a volatile write to REFCTL0.
    unsafe { (REFCTL0 as *mut u16).write_volatile(value) }
}

/// The converter.
pub struct Adc<'d> {
    _peri: Peri<'d, peripherals::ADC12>,
    config: Config,
}

impl<'d> Adc<'d> {
    /// Turn the converter on and configure it.
    ///
    /// If `config` asks for the internal reference this waits for it to settle, so the first
    /// conversion afterwards is already good.
    pub fn new(peri: Peri<'d, peripherals::ADC12>, config: Config) -> Self {
        if let Some(refvsel) = config.reference.refvsel() {
            write_ref((read_ref() & !0x0030) | (refvsel << 4) | REFON);
            // The generator takes tens of microseconds. Polling is simpler than a timer here and
            // this happens once, at start-up.
            while read_ref() & REFGENRDY == 0 {}
        }

        // ADC12ENC must be clear while the control registers are written; almost nothing in them is
        // writable otherwise.
        write(CTL0, 0);
        write(
            CTL0,
            ADC12ON | ((config.sample_time as u16) << 8) | ((config.sample_time as u16) << 12),
        );
        // ADC12SHP makes the sampling timer produce the sampling pulse, so a conversion is started
        // by one write rather than by holding a bit up for the sampling period.
        write(CTL1, ADC12SHP | (config.clock.bits() << 3));
        write(CTL2, (config.resolution as u16) << 4);
        // Sequences start at slot 0, which is the one slot this driver uses.
        write(CTL3, 0);

        Self { _peri: peri, config }
    }

    /// The reference this converter was configured with.
    pub fn reference(&self) -> Reference {
        self.config.reference
    }

    /// The largest reading this converter produces.
    pub fn max_value(&self) -> u16 {
        self.config.resolution.max_value()
    }

    /// Convert `raw` to millivolts, where the reference is a known voltage.
    ///
    /// `None` against `AVCC`, for the reason given in [`Reference::millivolts`].
    pub fn to_millivolts(&self, raw: u16) -> Option<u16> {
        let full_scale = self.config.reference.millivolts()?;
        let max = self.config.resolution.max_value() as u32;
        Some(((raw as u32 * full_scale as u32) / max) as u16)
    }

    /// Set up slot 0 for `channel` and start a conversion.
    fn start(&mut self, channel: u8) {
        write(CTL0, read(CTL0) & !ADC12ENC);
        write(MCTL0, (channel as u16 & 0x1f) | (self.config.reference.vrsel() << 8));
        write(IFGR0, 0);
        write(CTL0, read(CTL0) | ADC12ENC | ADC12SC);
    }

    /// Read `channel`, waiting for the conversion with the CPU spinning.
    pub fn blocking_read_channel(&mut self, channel: u8) -> u16 {
        write(IER0, 0);
        self.start(channel);
        while read(IFGR0) & 1 == 0 {}
        read(MEM0)
    }

    /// Read `channel`, letting other tasks run while the conversion happens.
    ///
    /// A conversion is a few microseconds, so this is worth it only when the sample time is long or
    /// there is genuinely something else to do.
    pub async fn read_channel(&mut self, channel: u8) -> u16 {
        self.start(channel);

        core::future::poll_fn(|cx| {
            if read(IFGR0) & 1 != 0 {
                return core::task::Poll::Ready(());
            }
            WAKER.register(cx.waker());
            // Registering first and then re-checking closes the window where the conversion
            // finishes between the check and the registration.
            if read(IFGR0) & 1 != 0 {
                return core::task::Poll::Ready(());
            }
            write(IER0, 1);
            core::task::Poll::Pending
        })
        .await;

        write(IER0, 0);
        read(MEM0)
    }

    /// Read a pin, blocking.
    pub fn blocking_read<P: AdcChannel>(&mut self, pin: &mut Peri<'_, P>) -> u16 {
        crate::gpio::set_analog(&**pin);
        self.blocking_read_channel(P::CHANNEL)
    }

    /// Read a pin.
    pub async fn read<P: AdcChannel>(&mut self, pin: &mut Peri<'_, P>) -> u16 {
        crate::gpio::set_analog(&**pin);
        self.read_channel(P::CHANNEL).await
    }

    /// Read one of the internal channels, blocking.
    pub fn blocking_read_internal(&mut self, channel: Channel) -> u16 {
        self.blocking_read_channel(channel.channel())
    }

    /// Read one of the internal channels.
    pub async fn read_internal(&mut self, channel: Channel) -> u16 {
        self.read_channel(channel.channel()).await
    }
}

impl Drop for Adc<'_> {
    fn drop(&mut self) {
        write(CTL0, 0);
        if self.config.reference.refvsel().is_some() {
            write_ref(read_ref() & !REFON);
        }
    }
}

embassy_executor::msp430_interrupt! {
    /// A conversion finished.
    ///
    /// The flag is left up as the answer the future polls for; what the handler does is mask the
    /// source, so it cannot re-enter before the result has been read.
    unsafe fn ADC12_B() {
        if read(IER0) & 1 == 0 || read(IFGR0) & 1 == 0 {
            return;
        }
        write(IER0, 0);
        WAKER.wake();
    }
}
