//! A ring of WS2812B (NeoPixel) LEDs, driven by PWM with EasyDMA so the CPU, and the SoftDevice, are
//! free while the bits go out.
//!
//! At 16 MHz with a period of 20 ticks each PWM period is one 1.25 µs WS2812B bit: high for 13 ticks
//! for a 1, 7 ticks for a 0. The frame ends with the line held low for more than 50 µs, which
//! latches the colours.
//!
//! The ring takes commands over two characteristics:
//!
//! - colours: 3 bytes `R G B` for the whole ring, or 3 bytes for each LED (the single byte 1 is
//!   still taken as Rainbow FTB, as it was before effects had their own characteristic);
//! - control: `id` or `id R G B` to start effect `id` (see [`Effect`]) in its own or the given
//!   colour, `80 level` for the brightness (255 = [`MAX_BRIGHTNESS`]), `81 speed` for the speed of
//!   the effects (100 = normal, 200 = twice as fast).

use embassy_futures::select::{Either, select};
use embassy_nrf::Peri;
use embassy_nrf::gpio::Pin as GpioPin;
use embassy_nrf::pwm::{
    Config, Instance, Prescaler, SequenceConfig, SequenceLoad, SequencePwm, SingleSequenceMode, SingleSequencer,
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Ticker, Timer};

pub const NUM_LEDS: usize = 8;

/// Every channel is scaled by this / 255. Eight LEDs at full white draw about 480 mA, close to what
/// the Feather's 3.3 V regulator gives; a quarter keeps the ring well inside it.
pub const MAX_BRIGHTNESS: u16 = 64;

pub type Rgb = [u8; 3];
pub type Colors = [Rgb; NUM_LEDS];

const BLACK: Rgb = [0, 0, 0];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Effect {
    /// All LEDs dark.
    Off = 0,
    /// The whole ring in one colour.
    Solid = 1,
    /// A dot running round the ring through the colours of the rainbow, its trail fading to black.
    RainbowFtb = 2,
    /// The rainbow spread round the ring, turning.
    Rainbow = 3,
    /// The whole ring going through the colours of the rainbow.
    RainbowCycle = 4,
    /// The colour fading in and out.
    Breathe = 5,
    /// A dot of the colour running round the ring with a fading tail.
    Comet = 6,
    /// Every fourth LED lit, the pattern moving round like a theatre marquee.
    TheaterChase = 7,
    /// The ring filling with the colour LED by LED, then emptying the same way.
    ColorWipe = 8,
    /// LEDs flashing up at random and fading; in random colours unless one is given.
    Sparkle = 9,
    /// Flickering orange and red.
    Fire = 10,
    /// Double flashes, red on one half of the ring, then blue on the other.
    Police = 11,
    /// A dot going back and forth over the ring with a fading tail.
    Scanner = 12,
    /// Short flashes of the colour.
    Strobe = 13,
}

impl Effect {
    const ALL: [Effect; 14] = [
        Effect::Off,
        Effect::Solid,
        Effect::RainbowFtb,
        Effect::Rainbow,
        Effect::RainbowCycle,
        Effect::Breathe,
        Effect::Comet,
        Effect::TheaterChase,
        Effect::ColorWipe,
        Effect::Sparkle,
        Effect::Fire,
        Effect::Police,
        Effect::Scanner,
        Effect::Strobe,
    ];

    pub fn from_id(id: u8) -> Option<Self> {
        Self::ALL.get(id as usize).copied()
    }

    /// Time between frames at speed 100, or None for a still picture.
    fn frame_ms(self) -> Option<u64> {
        Some(match self {
            Effect::Off | Effect::Solid => return None,
            Effect::RainbowFtb | Effect::Comet | Effect::Fire => 60,
            Effect::Rainbow => 20,
            Effect::RainbowCycle => 30,
            Effect::Breathe => 15,
            Effect::TheaterChase => 150,
            Effect::ColorWipe => 80,
            Effect::Sparkle => 40,
            Effect::Police => 50,
            Effect::Scanner => 70,
            Effect::Strobe => 30,
        })
    }

    /// The colour used when the command gives none. Black means the effect picks its own colours.
    fn default_color(self) -> Rgb {
        match self {
            Effect::Solid | Effect::Strobe => [255, 255, 255],
            Effect::Breathe | Effect::Comet => [0, 140, 255],
            Effect::TheaterChase => [255, 120, 0],
            Effect::ColorWipe => [0, 255, 40],
            Effect::Scanner => [255, 0, 0],
            _ => BLACK,
        }
    }
}

pub enum Command {
    /// Show these colours and stop any effect.
    Colors(Colors),
    Effect(Effect, Option<Rgb>),
    Brightness(u8),
    Speed(u8),
}

/// Commands for the ring, in the order they were written.
pub static COMMANDS: Channel<CriticalSectionRawMutex, Command, 8> = Channel::new();

/// Parses a write to the colours characteristic.
pub fn colors_command(bytes: &[u8]) -> Option<Command> {
    let mut colors = [BLACK; NUM_LEDS];
    match bytes {
        [1] => return Some(Command::Effect(Effect::RainbowFtb, None)),
        [r, g, b] => colors.fill([*r, *g, *b]),
        _ if bytes.len() == 3 * NUM_LEDS => {
            for (c, b) in colors.iter_mut().zip(bytes.chunks_exact(3)) {
                c.copy_from_slice(b);
            }
        }
        _ => return None,
    }
    Some(Command::Colors(colors))
}

/// Parses a write to the control characteristic.
pub fn control_command(bytes: &[u8]) -> Option<Command> {
    match *bytes {
        [0x80, level] => Some(Command::Brightness(level)),
        [0x81, speed] => Some(Command::Speed(speed.max(1))),
        [id] => Some(Command::Effect(Effect::from_id(id)?, None)),
        [id, r, g, b] => Some(Command::Effect(Effect::from_id(id)?, Some([r, g, b]))),
        _ => None,
    }
}

// The high bit inverts the polarity, so the duty is the time the line is high.
const T1H: u16 = 0x8000 | 13;
const T0H: u16 = 0x8000 | 7;
const RES: u16 = 0x8000;

/// The fully saturated colour at `hue` (0..=255 for once round the colour wheel).
fn wheel(hue: u8) -> Rgb {
    let x = (hue % 85) * 3;
    match hue / 85 {
        0 => [255 - x, x, 0],
        1 => [0, 255 - x, x],
        _ => [x, 0, 255 - x],
    }
}

fn scale(c: Rgb, level: u32) -> Rgb {
    c.map(|v| (v as u32 * level / 256) as u8)
}

/// Dims every LED to `keep` / 256 of its colour.
fn fade(colors: &mut Colors, keep: u32) {
    for c in colors.iter_mut() {
        *c = scale(*c, keep);
    }
}

/// xorshift32: random enough for flickering LEDs.
struct Rng(u32);

impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}

/// Draws frame `t` of `effect` into `colors`, which still hold the previous frame.
fn render(effect: Effect, color: Rgb, t: u32, colors: &mut Colors, rng: &mut Rng) {
    let n = NUM_LEDS as u32;
    match effect {
        Effect::Off => colors.fill(BLACK),
        Effect::Solid => colors.fill(color),
        Effect::RainbowFtb => {
            fade(colors, 160);
            colors[(t % n) as usize] = wheel((t * 3) as u8);
        }
        Effect::Rainbow => {
            for (i, c) in colors.iter_mut().enumerate() {
                *c = wheel((t * 2 + i as u32 * 256 / n) as u8);
            }
        }
        Effect::RainbowCycle => colors.fill(wheel((t * 2) as u8)),
        Effect::Breathe => {
            // A triangle wave, squared so the dark end lasts longer, as it does for the eye.
            let x = t * 4 % 512;
            let tri = if x < 256 { x } else { 511 - x };
            colors.fill(scale(color, tri * tri / 255 + 1));
        }
        Effect::Comet => {
            fade(colors, 150);
            colors[(t % n) as usize] = color;
        }
        Effect::TheaterChase => {
            for (i, c) in colors.iter_mut().enumerate() {
                *c = if (i as u32 + t) % 4 == 0 { color } else { BLACK };
            }
        }
        Effect::ColorWipe => {
            let step = t % (2 * n);
            for (i, c) in colors.iter_mut().enumerate() {
                let lit = if step < n {
                    i as u32 <= step
                } else {
                    i as u32 > step - n
                };
                *c = if lit { color } else { BLACK };
            }
        }
        Effect::Sparkle => {
            fade(colors, 200);
            if rng.next() % 3 == 0 {
                let i = (rng.next() % n) as usize;
                colors[i] = if color == BLACK { wheel(rng.next() as u8) } else { color };
            }
        }
        Effect::Fire => {
            for c in colors.iter_mut() {
                let heat = 120 + rng.next() % 136;
                let target = [heat as u8, (heat * heat / 255 / 3) as u8, 0];
                // Halfway to a new random heat each frame, so it flickers rather than blinks.
                for (v, t) in c.iter_mut().zip(target) {
                    *v = ((*v as u16 + t as u16) / 2) as u8;
                }
            }
        }
        Effect::Police => {
            let phase = t % 16;
            for (i, c) in colors.iter_mut().enumerate() {
                let left = (i as u32) < n / 2;
                *c = match phase {
                    0 | 2 if left => [255, 0, 0],
                    8 | 10 if !left => [0, 0, 255],
                    _ => BLACK,
                };
            }
        }
        Effect::Scanner => {
            let p = t % (2 * n - 2);
            let pos = if p < n { p } else { 2 * n - 2 - p };
            fade(colors, 130);
            colors[pos as usize] = color;
        }
        Effect::Strobe => colors.fill(if t % 10 == 0 { color } else { BLACK }),
    }
}

/// Carries out what arrives on [`COMMANDS`]. The ring starts dark.
pub async fn run<T: Instance>(pwm: Peri<'static, T>, pin: Peri<'static, impl GpioPin>) -> ! {
    let mut config = Config::default();
    config.sequence_load = SequenceLoad::Common;
    config.prescaler = Prescaler::Div1;
    config.max_duty = 20;
    let mut pwm = SequencePwm::new_1ch(pwm, pin, config).unwrap();

    let mut seq_config = SequenceConfig::default();
    // 40 more periods of RES after the final one: 51 µs low.
    seq_config.end_delay = 40;

    // Bits in the order the LEDs take them: G, R, B, most significant first. EasyDMA reads it from
    // RAM, which a task's state is.
    let mut words = [RES; NUM_LEDS * 24 + 1];
    let mut colors = [BLACK; NUM_LEDS];
    let mut effect = Effect::Off;
    let mut color = BLACK;
    let mut brightness = 255u32;
    let mut speed = 100u64;
    let mut ticker: Option<Ticker> = None;
    let mut t = 0u32;
    let mut rng = Rng(0x2545_f491);
    loop {
        for (led, [r, g, b]) in colors.iter().enumerate() {
            for (i, v) in [g, r, b].into_iter().enumerate() {
                let v = (*v as u32 * brightness * MAX_BRIGHTNESS as u32 / (255 * 255)) as u8;
                for bit in 0..8 {
                    words[led * 24 + i * 8 + bit] = if v & (0x80 >> bit) != 0 { T1H } else { T0H };
                }
            }
        }

        let seq = SingleSequencer::new(&mut pwm, &words, seq_config.clone());
        seq.start(SingleSequenceMode::Times(1)).unwrap();
        // The frame takes 193 × 1.25 µs plus the reset time, well under a millisecond. Dropping the
        // sequencer stops the PWM, which leaves the line low.
        Timer::after_millis(1).await;
        drop(seq);

        let command = match &mut ticker {
            None => Some(COMMANDS.receive().await),
            Some(ticker) => match select(COMMANDS.receive(), ticker.next()).await {
                Either::First(command) => Some(command),
                Either::Second(()) => None,
            },
        };
        let mut restart = false;
        match command {
            None => {
                t = t.wrapping_add(1);
                render(effect, color, t, &mut colors, &mut rng);
            }
            Some(Command::Colors(c)) => {
                colors = c;
                ticker = None;
            }
            Some(Command::Effect(e, c)) => {
                effect = e;
                color = c.unwrap_or(e.default_color());
                t = 0;
                colors.fill(BLACK);
                render(effect, color, t, &mut colors, &mut rng);
                restart = true;
            }
            Some(Command::Brightness(level)) => brightness = level as u32,
            Some(Command::Speed(s)) => {
                speed = s as u64;
                restart = ticker.is_some();
            }
        }
        if restart {
            ticker = effect
                .frame_ms()
                .map(|ms| Ticker::every(Duration::from_millis((ms * 100 / speed).max(10))));
        }
    }
}
