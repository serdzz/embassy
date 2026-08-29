//! The ultrasonic sensing front end: `UUPS`, `HSPLL`, `SAPH_A` and `SDHS`.
//!
//! Four modules that work as one. `UUPS` powers the analog section, `HSPLL` makes the fast clock
//! the converter needs, `SAPH_A` drives the transducer and switches the bias and the input
//! multiplexer, and `SDHS` digitises what comes back — writing samples straight into memory through
//! its own transfer controller, because at eight megasamples a second nothing else could keep up.
//!
//! What this gives you is **a captured waveform**: excite the transducer, record what the far
//! transducer heard, hand back the samples. Doing that in both directions and comparing the two is
//! how a flow meter works, and [`tof`] has the arithmetic for the comparison.
//!
//! # What this is not
//!
//! It is not TI's Ultrasonic Sensing Software Library, and it will not match it.
//!
//! That library does the same capture and then a great deal more: it runs the correlation on the
//! `LEA` accelerator, tracks the envelope across temperature, compensates for zero-flow drift, and
//! ships with a calibration workflow against a real flow rig. Its accuracy claims rest on all of
//! that, not on the capture. What is here is the capture, plus an honest estimator in [`tof`] that
//! will get you a time-of-flight difference good enough to see flow and not good enough to bill
//! anyone for it.
//!
//! Two more gaps worth naming. The driver does not load the factory drive-strength trims from the
//! `TLV` table into `CH0PUT`/`CH0PDT`/`CH0TT` and their channel-1 counterparts, so the output
//! drivers run at their reset defaults. And nothing here has been near a transducer.
//!
//! # Clocks you have to supply
//!
//! [`Config`] asks for `pll_hz` rather than working it out from the crystal and the multiplier. The
//! relationship between `PLLM` and what the PLL produces is in TI's documentation and in what its
//! design centre reports for a given board, and this driver would rather be told than guess: every
//! sample interval and every excitation period below is derived from that number, so a wrong guess
//! would be wrong everywhere at once and silently.

use embassy_hal_internal::Peri;
use embassy_sync::waitqueue::AtomicWaker;

use crate::peripherals;

pub mod tof;

const UUPS_BASE: u16 = 0x0ec0;
const HSPLL_BASE: u16 = 0x0ee0;
const SAPH_BASE: u16 = 0x0e00;
const SDHS_BASE: u16 = 0x0e80;

// UUPS.
const UUPSCTL: u16 = 0x10;
const LDORDY: u16 = 1 << 0;
const USS_BUSY: u16 = 1 << 3;
const USSSWRST: u16 = 1 << 7;
const USSPWRUP: u16 = 1 << 8;
const USSPWRDN: u16 = 1 << 14;
const USSSTOP: u16 = 1 << 15;

// HSPLL.
const HSPLLCTL: u16 = 0x10;
const HSPLLUSSXTLCTL: u16 = 0x12;
const PLL_LOCK: u16 = 1 << 0;
const PLLINFREQ: u16 = 1 << 8;
const USSXTEN: u16 = 1 << 0;
const OSCSTATE: u16 = 1 << 1;

// SDHS.
const SDHSRIS: u16 = 0x04;
const SDHSIMSC: u16 = 0x06;
const SDHSICR: u16 = 0x08;
const SDHSCTL0: u16 = 0x10;
const SDHSCTL1: u16 = 0x12;
const SDHSCTL2: u16 = 0x14;
const SDHSCTL3: u16 = 0x16;
const SDHSCTL4: u16 = 0x18;
const SDHSCTL6: u16 = 0x1c;
const SDHSDTCDA: u16 = 0x28;
const OVF: u16 = 1 << 0;
const ACQDONE: u16 = 1 << 1;
const SDHSON: u16 = 1 << 0;
const TRIGEN: u16 = 1 << 0;
const DTCOFF: u16 = 1 << 15;

// SAPH_A.
const SAPHRIS: u16 = 0x04;
const SAPHIMSC: u16 = 0x06;
const SAPHICR: u16 = 0x08;
const SAPHOCTL1: u16 = 0x14;
const SAPHOSEL: u16 = 0x16;
const SAPHICTL0: u16 = 0x30;
const SAPHBCTL: u16 = 0x34;
const SAPHPGC: u16 = 0x40;
const SAPHPGLPER: u16 = 0x42;
const SAPHPGHPER: u16 = 0x44;
const SAPHPGCTL: u16 = 0x46;
const SAPHASCTL0: u16 = 0x60;
const SAPHASCTL1: u16 = 0x62;
const SAPHASQTRIG: u16 = 0x64;
const DATAERR: u16 = 1 << 0;
const ASQSTOP: u16 = 1 << 7;

/// Woken by whichever of the four modules finished first; which one is worked out from the flags.
static WAKER: AtomicWaker = AtomicWaker::new();

/// What went wrong.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The USSXT oscillator never started. Usually a crystal that is not fitted or not oscillating.
    OscillatorFailed,
    /// The PLL never locked.
    PllUnlocked,
    /// The analog supply never came up.
    SupplyFailed,
    /// The requested excitation frequency cannot be made from `pll_hz`: the pulse generator counts
    /// half-periods in an 8-bit register, so the ratio has to land between 2 and 510.
    UnachievableExcitation,
    /// More samples were asked for than the sample-size counter holds, or than fit in the buffer.
    TooManySamples,
    /// The converter overran: samples arrived faster than they were stored.
    Overflow,
    /// The sequencer reported a data error.
    SequenceFailed,
}

/// Which transducer is being driven.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Channel {
    /// Channel 0 transmits, channel 1 listens.
    Ch0,
    /// Channel 1 transmits, channel 0 listens.
    Ch1,
}

impl Channel {
    const fn bits(self) -> u16 {
        match self {
            Channel::Ch0 => 0,
            Channel::Ch1 => 1,
        }
    }

    /// The other one.
    pub const fn opposite(self) -> Channel {
        match self {
            Channel::Ch0 => Channel::Ch1,
            Channel::Ch1 => Channel::Ch0,
        }
    }
}

/// How many modulator cycles go into one output sample.
///
/// The converter runs its modulator at the PLL clock and decimates. A higher ratio is quieter and
/// slower: the sample rate is the PLL clock divided by this.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum Oversampling {
    _10 = 0,
    _20 = 1,
    _40 = 2,
    #[default]
    _80 = 3,
    _160 = 4,
}

impl Oversampling {
    /// The ratio itself.
    pub const fn ratio(self) -> u32 {
        match self {
            Oversampling::_10 => 10,
            Oversampling::_20 => 20,
            Oversampling::_40 => 40,
            Oversampling::_80 => 80,
            Oversampling::_160 => 160,
        }
    }
}

/// Bits per sample.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum Resolution {
    _12Bit = 0,
    _13Bit = 1,
    #[default]
    _14Bit = 2,
}

/// Bias voltage applied to the transmitting transducer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum ExcitationBias {
    _0V2 = 0,
    #[default]
    _0V3 = 1,
    _0V4 = 2,
    _0V6 = 3,
}

/// Bias voltage at the amplifier input, which sets where the received signal sits.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum PgaBias {
    _0V75 = 0,
    #[default]
    _0V80 = 1,
    _0V90 = 2,
    _0V95 = 3,
}

/// How the front end is set up.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// The crystal on `USSXTIN`/`USSXTOUT`.
    pub xt_hz: u32,
    /// `PLLM` register value.
    pub pllm: u8,
    /// What the PLL produces — see the note at the top of this module on why this is given rather
    /// than derived. Everything timed here comes from it.
    pub pll_hz: u32,
    /// Transducer frequency, which the pulse generator is set to.
    pub excitation_hz: u32,
    /// How many excitation pulses per ping.
    pub pulses: u8,
    /// Braking pulses after the excitation, which damp the transducer so it stops ringing before
    /// the echo arrives. Zero if the transducer does not need it.
    pub stop_pulses: u8,
    /// Decimation ratio.
    pub oversampling: Oversampling,
    /// Bits per sample.
    pub resolution: Resolution,
    /// Amplifier gain, as the register code. The mapping to decibels is in the datasheet.
    pub pga_gain: u8,
    /// Bias on the driven transducer.
    pub excitation_bias: ExcitationBias,
    /// Bias at the amplifier input.
    pub pga_bias: PgaBias,
    /// Samples to discard at the start of a capture, before the converter has settled.
    pub settling_samples: u8,
}

impl Default for Config {
    /// A starting point for a 1 MHz water transducer on an 8 MHz crystal.
    ///
    /// Every one of these is a guess about somebody else's plumbing. Treat it as something that
    /// compiles, not as something that measures.
    fn default() -> Self {
        Self {
            xt_hz: 8_000_000,
            pllm: 19,
            pll_hz: 80_000_000,
            excitation_hz: 1_000_000,
            pulses: 10,
            stop_pulses: 2,
            oversampling: Oversampling::_20,
            resolution: Resolution::_14Bit,
            pga_gain: 0,
            excitation_bias: ExcitationBias::_0V3,
            pga_bias: PgaBias::_0V80,
            settling_samples: 0,
        }
    }
}

impl Config {
    /// Samples per second, which is what turns a sample index into a time.
    pub const fn sample_rate_hz(&self) -> u32 {
        self.pll_hz / self.oversampling.ratio()
    }

    /// Picoseconds per sample.
    ///
    /// Picoseconds because at eighty megasamples a second a sample is twelve and a half
    /// nanoseconds, and the whole business of measuring flow is resolving a fraction of that.
    pub const fn sample_period_ps(&self) -> u32 {
        1_000_000_000_000u64.div_ceil(self.sample_rate_hz() as u64) as u32
    }
}

#[inline]
fn read(base: u16, offset: u16) -> u16 {
    // SAFETY: a volatile read of a USS register.
    unsafe { ((base + offset) as *mut u16).read_volatile() }
}

#[inline]
fn write(base: u16, offset: u16, value: u16) {
    // SAFETY: a volatile write to a USS register.
    unsafe { ((base + offset) as *mut u16).write_volatile(value) }
}

/// Stop the compiler moving memory accesses across this point.
///
/// `core::sync::atomic::compiler_fence` would be the obvious thing, and it does not compile for
/// this target — LLVM's MSP430 backend cannot select an `AtomicFence` node at all. An empty `asm!`
/// block does the same job here: without `nomem` it is assumed to touch memory, so nothing can be
/// hoisted across it, and it emits no instructions.
///
/// A compiler barrier is all that is needed. This is one core with no caches and no store buffer;
/// what has to be prevented is the compiler keeping a copy of the buffer in registers across the
/// wait, not the hardware reordering anything.
#[inline(always)]
fn memory_barrier() {
    // SAFETY: an empty assembly block. It has no operands and no effects beyond being a barrier.
    unsafe { core::arch::asm!("", options(nostack, preserves_flags)) }
}

/// Spin until `f` is true, giving up after a bounded number of tries.
///
/// Bounded because every one of these waits is on a piece of analog hardware that can simply fail
/// to start — an unfitted crystal, a PLL that will not lock — and an unbounded spin there is a dead
/// machine rather than a reported fault.
fn wait_until(mut f: impl FnMut() -> bool) -> bool {
    // Generous: the slowest of these is the crystal, which needs a millisecond or so, and this
    // loop is a few cycles.
    for _ in 0..200_000u32 {
        if f() {
            return true;
        }
    }
    false
}

/// The ultrasonic front end.
pub struct Uss<'d> {
    _uups: Peri<'d, peripherals::UUPS>,
    _hspll: Peri<'d, peripherals::HSPLL>,
    _saph: Peri<'d, peripherals::SAPH>,
    _sdhs: Peri<'d, peripherals::SDHS>,
    config: Config,
}

impl<'d> Uss<'d> {
    /// Power the front end up and configure it.
    ///
    /// All four modules are taken together because they are not independently useful: driving a
    /// transducer without the converter, or the converter without its clock, is not something worth
    /// letting a caller express.
    pub fn new(
        uups: Peri<'d, peripherals::UUPS>,
        hspll: Peri<'d, peripherals::HSPLL>,
        saph: Peri<'d, peripherals::SAPH>,
        sdhs: Peri<'d, peripherals::SDHS>,
        config: Config,
    ) -> Result<Self, Error> {
        // Checked before anything is powered on, so an unachievable excitation frequency is a
        // rejected call rather than a front end left running.
        pulse_periods(&config)?;

        let mut uss = Self {
            _uups: uups,
            _hspll: hspll,
            _saph: saph,
            _sdhs: sdhs,
            config,
        };

        uss.bring_up()?;
        Ok(uss)
    }

    /// Bring the front end back up after [`Uss::power_down`].
    ///
    /// Powering down does not just gate a clock: it drops the analog supply and stops the crystal,
    /// and everything configured in the four modules goes with them. So this is the whole sequence
    /// again, not a resume — which is worth knowing, because it costs the crystal's start-up time,
    /// a millisecond or so, every time.
    ///
    /// A meter that measures once a second and sleeps in between should still do this: a
    /// millisecond of oscillator against a second of powered-down analog front end is a trade worth
    /// making on a battery.
    pub fn restart(&mut self) -> Result<(), Error> {
        self.bring_up()
    }

    /// The bring-up sequence, shared by construction and [`Uss::restart`].
    fn bring_up(&mut self) -> Result<(), Error> {
        let (lper, hper) = pulse_periods(&self.config)?;
        self.power_up()?;
        self.start_clock()?;
        self.configure_converter();
        self.configure_front_end(lper, hper);
        Ok(())
    }

    /// Bring up the analog supply.
    fn power_up(&mut self) -> Result<(), Error> {
        // A reset first, so that a warm restart does not inherit half a configuration.
        write(UUPS_BASE, UUPSCTL, USSSWRST);
        write(UUPS_BASE, UUPSCTL, USSPWRUP);

        if !wait_until(|| read(UUPS_BASE, UUPSCTL) & LDORDY != 0) {
            write(UUPS_BASE, UUPSCTL, USSPWRDN);
            return Err(Error::SupplyFailed);
        }
        Ok(())
    }

    /// Start the crystal and lock the PLL.
    fn start_clock(&mut self) -> Result<(), Error> {
        write(HSPLL_BASE, HSPLLUSSXTLCTL, USSXTEN);
        if !wait_until(|| read(HSPLL_BASE, HSPLLUSSXTLCTL) & OSCSTATE != 0) {
            return Err(Error::OscillatorFailed);
        }

        // PLLINFREQ tells the loop which side of 6 MHz its input is on.
        let infreq = if self.config.xt_hz > 6_000_000 { PLLINFREQ } else { 0 };
        write(HSPLL_BASE, HSPLLCTL, infreq | ((self.config.pllm as u16 & 0x3f) << 10));

        if !wait_until(|| read(HSPLL_BASE, HSPLLCTL) & PLL_LOCK != 0) {
            return Err(Error::PllUnlocked);
        }
        Ok(())
    }

    /// Set the converter up. The sample count and the destination are per capture, not here.
    fn configure_converter(&mut self) {
        write(SDHS_BASE, SDHSCTL4, SDHSON);
        write(SDHS_BASE, SDHSCTL1, self.config.oversampling as u16);

        // Two's complement, right-aligned, triggered by the front end rather than by software, and
        // the first few samples discarded while the converter settles.
        const TRGSRC_SAPH: u16 = 1 << 15;
        write(
            SDHS_BASE,
            SDHSCTL0,
            TRGSRC_SAPH
                | ((self.config.resolution as u16) << 10)
                | ((self.config.settling_samples as u16 & 0x07) << 1),
        );

        write(SDHS_BASE, SDHSCTL6, self.config.pga_gain as u16 & 0x3f);
    }

    /// Set the analog front end up: outputs, bias, and the pulse generator.
    fn configure_front_end(&mut self, lper: u8, hper: u8) {
        // Both channel outputs are driven by the pulse generator, at full strength.
        const PPG_DRIVES_BOTH: u16 = (1 << 0) | (1 << 2);
        write(SAPH_BASE, SAPHOSEL, PPG_DRIVES_BOTH);
        const CH0FP: u16 = 1 << 8;
        const CH1FP: u16 = 1 << 9;
        write(SAPH_BASE, SAPHOCTL1, CH0FP | CH1FP);

        // The sequencer owns the bias switches and the input multiplexer, so that the listening
        // channel is biased and selected without the CPU being involved in the microseconds
        // between transmitting and receiving.
        const ASQBSC: u16 = 1 << 0;
        write(
            SAPH_BASE,
            SAPHBCTL,
            ASQBSC
                | ((self.config.excitation_bias as u16) << 4)
                | ((self.config.pga_bias as u16) << 6),
        );
        const MUXCTL_ASQ: u16 = 1 << 4;
        write(SAPH_BASE, SAPHICTL0, MUXCTL_ASQ);

        // The pulse train itself.
        write(SAPH_BASE, SAPHPGLPER, lper as u16);
        write(SAPH_BASE, SAPHPGHPER, hper as u16);
        write(
            SAPH_BASE,
            SAPHPGC,
            (self.config.pulses.saturating_sub(1) as u16 & 0x7f)
                | ((self.config.stop_pulses as u16 & 0x0f) << 8),
        );

        // Channel and trigger both come from the sequencer.
        const PGSEL_ASQ: u16 = 1 << 0;
        const TRSEL_ASQ: u16 = 1 << 4;
        const PPGEN: u16 = 1 << 9;
        write(SAPH_BASE, SAPHPGCTL, PGSEL_ASQ | TRSEL_ASQ | PPGEN);
    }

    /// Capture one ping into `buf`, and return how many samples landed there.
    ///
    /// `buf` is written by the converter's transfer controller, not by the CPU. If this future is
    /// dropped before it finishes, the capture is stopped before the borrow ends — otherwise the
    /// hardware would carry on writing into memory the caller has taken back.
    pub async fn capture(&mut self, channel: Channel, buf: &mut [i16]) -> Result<usize, Error> {
        if buf.is_empty() || buf.len() > 0x03ff {
            return Err(Error::TooManySamples);
        }

        self.arm(channel, buf, 1, false);

        // From here to the end of the wait, `buf` belongs to the hardware. The guard stops the
        // capture on every exit path, including this future being dropped mid-flight.
        let _guard = CaptureGuard;

        wait_for_capture().await?;
        drop(_guard);

        // The samples were written by hardware behind the compiler's back; this keeps it from
        // hoisting reads of `buf` above the wait.
        memory_barrier();
        Ok(buf.len())
    }

    /// Capture in both directions, one after the other, without the CPU in between.
    ///
    /// The sequencer swaps the transmitting channel itself between the two pings, so the gap
    /// between them is the hardware's, not a task's. That matters: the whole measurement is the
    /// difference between the two flight times, and anything that varies between them lands
    /// directly in the answer.
    ///
    /// One buffer, split down the middle, because that is what the hardware does — the transfer
    /// controller is given a single destination for the whole sequence and fills it straight
    /// through. `buf` must have an even length. The returned halves are the ping `first`
    /// transmitted and then the other one, in that order.
    pub async fn capture_pair<'b>(
        &mut self,
        first: Channel,
        buf: &'b mut [i16],
    ) -> Result<(&'b [i16], &'b [i16]), Error> {
        if buf.is_empty() || buf.len() % 2 != 0 || buf.len() > 0x03ff {
            return Err(Error::TooManySamples);
        }
        let half = buf.len() / 2;

        self.arm(first, buf, 2, true);
        let _guard = CaptureGuard;
        wait_for_capture().await?;
        drop(_guard);
        memory_barrier();

        Ok(buf.split_at(half))
    }

    /// Point the converter at `buf`, arm it, and start the sequencer.
    /// `buf` is the whole destination: the transfer controller fills it straight through, however
    /// many pings that takes.
    fn arm(&mut self, channel: Channel, buf: &mut [i16], pings: u16, toggle: bool) {
        let samples = buf.len() as u16;

        write(SDHS_BASE, SDHSICR, 0xffff);
        write(SAPH_BASE, SAPHICR, 0xffff);

        // DTCOFF clear: the transfer controller stores each sample, which is the only way to keep
        // up at these rates.
        write(SDHS_BASE, SDHSCTL2, samples & 0x03ff);
        write(SDHS_BASE, SDHSDTCDA, buf.as_mut_ptr() as u16);
        write(SDHS_BASE, SDHSCTL3, TRIGEN);

        // How many pings, which channel starts, and whether to swap between them.
        write(
            SAPH_BASE,
            SAPHASCTL1,
            if toggle { 1 << 0 } else { 0 },
        );
        const ASQTEN: u16 = 1 << 9;
        write(
            SAPH_BASE,
            SAPHASCTL0,
            ASQTEN | (pings.saturating_sub(1) & 0x03) | (channel.bits() << 4),
        );

        write(SDHS_BASE, SDHSIMSC, ACQDONE | OVF);
        write(SAPH_BASE, SAPHIMSC, DATAERR);
        write(SAPH_BASE, SAPHASQTRIG, 1);
    }

    /// The configuration this front end is running.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Turn the analog section off.
    ///
    /// Worth doing between measurements: the bias and the amplifier are the expensive part of a
    /// battery-powered meter, and a meter measures for microseconds a second.
    pub fn power_down(&mut self) {
        stop_capture();
        // Cutting the supply while a measurement is still unwinding leaves the analog section in a
        // state that only a reset clears, so wait for it to say it is idle. Bounded, because a
        // module that never goes idle must not take the caller down with it.
        wait_until(|| read(UUPS_BASE, UUPSCTL) & USS_BUSY == 0);
        write(SDHS_BASE, SDHSCTL4, 0);
        write(HSPLL_BASE, HSPLLUSSXTLCTL, 0);
        write(UUPS_BASE, UUPSCTL, USSPWRDN);
    }
}

impl Drop for Uss<'_> {
    fn drop(&mut self) {
        self.power_down();
    }
}

/// Stop a capture that is under way, whatever state it is in.
fn stop_capture() {
    // `USSSTOP` is the module's own way of abandoning a measurement, and it unwinds the analog
    // section in the right order. The rest below is belt and braces for the case where the
    // sequencer was never started.
    write(UUPS_BASE, UUPSCTL, read(UUPS_BASE, UUPSCTL) | USSSTOP);
    write(SAPH_BASE, SAPHASCTL0, ASQSTOP);
    write(SAPH_BASE, SAPHPGCTL, read(SAPH_BASE, SAPHPGCTL) | (1 << 15));
    write(SDHS_BASE, SDHSCTL3, 0);
    write(SDHS_BASE, SDHSCTL2, read(SDHS_BASE, SDHSCTL2) | DTCOFF);
    write(SDHS_BASE, SDHSIMSC, 0);
    write(SAPH_BASE, SAPHIMSC, 0);
    write(SDHS_BASE, SDHSICR, 0xffff);
    write(SAPH_BASE, SAPHICR, 0xffff);
}

/// Stops the capture when it goes out of scope, however that happens.
struct CaptureGuard;

impl Drop for CaptureGuard {
    fn drop(&mut self) {
        stop_capture();
    }
}

/// Wait for the converter to say the samples are in memory.
async fn wait_for_capture() -> Result<(), Error> {
    core::future::poll_fn(|cx| {
        let done = |flags: u16| flags & (ACQDONE | OVF) != 0;

        if done(read(SDHS_BASE, SDHSRIS)) || read(SAPH_BASE, SAPHRIS) & DATAERR != 0 {
            return core::task::Poll::Ready(());
        }
        WAKER.register(cx.waker());
        // Re-check after registering, so a capture that finished in between is not waited on
        // forever.
        if done(read(SDHS_BASE, SDHSRIS)) || read(SAPH_BASE, SAPHRIS) & DATAERR != 0 {
            return core::task::Poll::Ready(());
        }
        write(SDHS_BASE, SDHSIMSC, ACQDONE | OVF);
        write(SAPH_BASE, SAPHIMSC, DATAERR);
        core::task::Poll::Pending
    })
    .await;

    let sdhs = read(SDHS_BASE, SDHSRIS);
    if read(SAPH_BASE, SAPHRIS) & DATAERR != 0 {
        return Err(Error::SequenceFailed);
    }
    if sdhs & OVF != 0 {
        return Err(Error::Overflow);
    }
    Ok(())
}

/// Low and high phase counts for the excitation frequency.
///
/// The pulse generator counts the PLL clock through two 8-bit registers, one per half-cycle, so the
/// reachable frequencies are `pll_hz` divided by an even number from 2 to 510. A square wave means
/// the two halves are equal; an odd total is split as evenly as it can be.
fn pulse_periods(config: &Config) -> Result<(u8, u8), Error> {
    if config.excitation_hz == 0 {
        return Err(Error::UnachievableExcitation);
    }
    let total = config.pll_hz / config.excitation_hz;
    if !(2..=510).contains(&total) {
        return Err(Error::UnachievableExcitation);
    }
    let low = (total / 2) as u8;
    let high = (total - total / 2) as u8;
    Ok((low, high))
}

embassy_executor::msp430_interrupt! {
    /// The converter finished a capture, or overran.
    unsafe fn SDHS() {
        if read(SDHS_BASE, SDHSIMSC) & read(SDHS_BASE, SDHSRIS) == 0 {
            return;
        }
        // Masked, not cleared: the flag is the answer the future polls for, and clearing it here
        // would lose which of the two happened.
        write(SDHS_BASE, SDHSIMSC, 0);
        WAKER.wake();
    }

    /// The sequencer finished, or gave up.
    unsafe fn SAPH_A() {
        if read(SAPH_BASE, SAPHIMSC) & read(SAPH_BASE, SAPHRIS) == 0 {
            return;
        }
        write(SAPH_BASE, SAPHIMSC, 0);
        WAKER.wake();
    }

    /// The analog supply had something to say — a power-up timeout, or a request it could not
    /// honour. Nothing waits on these; they are masked so the flag cannot spin the CPU.
    unsafe fn UUPS() {
        write(UUPS_BASE, 0x06, 0);
        write(UUPS_BASE, 0x08, 0xffff);
    }

    /// The PLL lost lock. Same treatment: masked and cleared, and the next capture will fail its
    /// own check rather than being told from here.
    unsafe fn HSPLL() {
        write(HSPLL_BASE, 0x06, 0);
        write(HSPLL_BASE, 0x08, 0xffff);
    }
}
