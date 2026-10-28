//! A ring of WS2812B (NeoPixel) LEDs, driven by PWM with EasyDMA so the CPU, and the SoftDevice, are
//! free while the bits go out.
//!
//! At 16 MHz with a period of 20 ticks each PWM period is one 1.25 µs WS2812B bit: high for 13 ticks
//! for a 1, 7 ticks for a 0. The frame ends with the line held low for more than 50 µs, which
//! latches the colours.

use embassy_futures::select::{Either, select};
use embassy_nrf::Peri;
use embassy_nrf::gpio::Pin as GpioPin;
use embassy_nrf::pwm::{
    Config, Instance, Prescaler, SequenceConfig, SequenceLoad, SequencePwm, SingleSequenceMode, SingleSequencer,
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Ticker, Timer};

pub const NUM_LEDS: usize = 8;

/// Every channel is scaled by this / 255. Eight LEDs at full white draw about 480 mA, close to what
/// the Feather's 3.3 V regulator gives; a quarter keeps the ring well inside it.
pub const MAX_BRIGHTNESS: u16 = 64;

pub type Colors = [[u8; 3]; NUM_LEDS];

pub enum Command {
    /// Show these colours (R, G, B per LED) and stop any effect.
    Colors(Colors),
    /// A dot running round the ring through the colours of the rainbow, its trail fading to black.
    RainbowFtb,
}

/// What the ring should do next.
pub static COMMAND: Signal<CriticalSectionRawMutex, Command> = Signal::new();

// The high bit inverts the polarity, so the duty is the time the line is high.
const T1H: u16 = 0x8000 | 13;
const T0H: u16 = 0x8000 | 7;
const RES: u16 = 0x8000;

/// Rainbow FTB: one step of the dot per frame, and how much of each colour is left a frame later.
const FTB_FRAME: Duration = Duration::from_millis(60);
const FTB_FADE: u16 = 160; // / 256
const FTB_HUE_STEP: u8 = 3;

/// Handles a GATT write: 3 bytes (R, G, B) for the whole ring, 3 bytes for each LED, or the single
/// byte 1 for Rainbow FTB. Returns false for anything else.
pub fn command_from_bytes(bytes: &[u8]) -> bool {
    let mut colors = [[0; 3]; NUM_LEDS];
    match bytes {
        [1] => {
            COMMAND.signal(Command::RainbowFtb);
            return true;
        }
        [r, g, b] => colors.fill([*r, *g, *b]),
        _ if bytes.len() == 3 * NUM_LEDS => {
            for (c, b) in colors.iter_mut().zip(bytes.chunks_exact(3)) {
                c.copy_from_slice(b);
            }
        }
        _ => return false,
    }
    COMMAND.signal(Command::Colors(colors));
    true
}

/// The fully saturated colour at `hue` (0..=255 for once round the colour wheel).
fn wheel(hue: u8) -> [u8; 3] {
    let x = (hue % 85) * 3;
    match hue / 85 {
        0 => [255 - x, x, 0],
        1 => [0, 255 - x, x],
        _ => [x, 0, 255 - x],
    }
}

/// Carries out what arrives on [`COMMAND`]. The ring starts dark.
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
    let mut colors = [[0u8; 3]; NUM_LEDS];
    let mut effect: Option<(Ticker, usize, u8)> = None;
    loop {
        for (led, [r, g, b]) in colors.iter().enumerate() {
            for (i, v) in [g, r, b].into_iter().enumerate() {
                let v = (*v as u16 * MAX_BRIGHTNESS / 255) as u8;
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

        let command = match &mut effect {
            None => Some(COMMAND.wait().await),
            Some((ticker, _, _)) => match select(COMMAND.wait(), ticker.next()).await {
                Either::First(command) => Some(command),
                Either::Second(()) => None,
            },
        };
        match command {
            Some(Command::Colors(c)) => {
                colors = c;
                effect = None;
            }
            Some(Command::RainbowFtb) => effect = Some((Ticker::every(FTB_FRAME), 0, 0)),
            None => {}
        }
        if let Some((_, pos, hue)) = &mut effect {
            for c in colors.iter_mut().flatten() {
                *c = (*c as u16 * FTB_FADE / 256) as u8;
            }
            colors[*pos] = wheel(*hue);
            *pos = (*pos + 1) % NUM_LEDS;
            *hue = hue.wrapping_add(FTB_HUE_STEP);
        }
    }
}
