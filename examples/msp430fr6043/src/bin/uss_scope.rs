//! Dump a captured ultrasonic waveform over the UART, so it can be looked at.
//!
//! This is the first thing to run on a new board, before `flow` and before believing any number a
//! meter produces. `flow` reports a difference in flight time you have no way to check; this prints
//! the samples the receiving transducer actually produced, and a plot of those answers the
//! questions that matter at bring-up:
//!
//! * Is the transducer ringing at all, or is the excitation not reaching it?
//! * Where in the capture does the burst arrive — is the window even looking at the right time?
//! * How big is it? That sets the gain and the threshold `flow` needs.
//! * Has the transmitting transducer stopped ringing before the echo lands, or do the stop pulses
//!   need adjusting?
//!
//! Output is one sample per line, so it pastes straight into anything that plots a column of
//! numbers. Each capture is preceded by a header line naming the direction and the sample period,
//! and followed by a summary with the peak and a threshold to try.
//!
//! At 115200 baud a 200-sample dump takes about a tenth of a second, which is why this runs on
//! demand rather than continuously.
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_msp430::clock::DcoFreq;
use embassy_msp430::uart::{Config as UartConfig, Uart};
use embassy_msp430::uss::{Channel, Config as UssConfig, Uss};
use embassy_time::{Duration, Timer};

use panic_msp430 as _;

/// Samples per ping.
const SAMPLES: usize = 200;

/// Seconds between captures. Long enough to read the last one.
const PERIOD_S: u64 = 5;

/// A signed decimal into `out`, returning the part that was written.
fn dec(value: i32, out: &mut [u8; 8]) -> &[u8] {
    let negative = value < 0;
    let mut v = value.unsigned_abs();
    let mut i = out.len();
    loop {
        i -= 1;
        out[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 || i == 1 {
            break;
        }
    }
    if negative {
        i -= 1;
        out[i] = b'-';
    }
    &out[i..]
}

/// Print one capture, one sample per line, and return its peak magnitude.
async fn dump(uart: &mut Uart<'_>, label: &[u8], samples: &[i16]) -> u16 {
    let mut digits = [0u8; 8];
    let mut peak = 0u16;

    uart.write(b"# ").await.unwrap();
    uart.write(label).await.unwrap();
    uart.write(b"\r\n").await.unwrap();

    for &s in samples {
        peak = peak.max(s.unsigned_abs());
        uart.write(dec(s as i32, &mut digits)).await.unwrap();
        uart.write(b"\r\n").await.unwrap();
    }
    peak
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_msp430::Config::default();
    config.clock.dco = DcoFreq::_8MHz;
    let p = embassy_msp430::init(config);

    let mut uart = Uart::new(p.EUSCI_A0, p.P4_4, p.P4_3, UartConfig::default()).unwrap();

    let uss_config = UssConfig::default();
    let mut uss = match Uss::new(p.UUPS, p.HSPLL, p.SAPH, p.SDHS, uss_config) {
        Ok(uss) => uss,
        Err(_) => {
            // Worth stopping on. Every number below would be meaningless, and printing them anyway
            // is how a board gets declared working when nothing is connected to it.
            uart.write(b"USS did not start: check the crystal and the supply\r\n")
                .await
                .unwrap();
            loop {
                Timer::after(Duration::from_secs(60)).await;
            }
        }
    };

    let mut digits = [0u8; 8];
    uart.write(b"# sample period, ps: ").await.unwrap();
    uart.write(dec(uss_config.sample_period_ps() as i32, &mut digits))
        .await
        .unwrap();
    uart.write(b"\r\n").await.unwrap();

    let mut buf = [0i16; SAMPLES * 2];

    loop {
        if uss.restart().is_err() {
            uart.write(b"# USS did not restart\r\n").await.unwrap();
            Timer::after(Duration::from_secs(PERIOD_S)).await;
            continue;
        }

        match uss.capture_pair(Channel::Ch0, &mut buf).await {
            Ok((up, down)) => {
                let peak_up = dump(&mut uart, b"ch0 transmitting", up).await;
                let peak_down = dump(&mut uart, b"ch1 transmitting", down).await;

                // Half the smaller peak is a reasonable first threshold: comfortably above the
                // noise, comfortably below the burst, and the same on both directions — which
                // matters, because a threshold that catches one direction earlier than the other
                // puts a fixed error straight into the flight-time difference.
                uart.write(b"# peaks ").await.unwrap();
                uart.write(dec(peak_up as i32, &mut digits)).await.unwrap();
                uart.write(b" ").await.unwrap();
                uart.write(dec(peak_down as i32, &mut digits)).await.unwrap();
                uart.write(b", try threshold ").await.unwrap();
                uart.write(dec((peak_up.min(peak_down) / 2) as i32, &mut digits))
                    .await
                    .unwrap();
                uart.write(b"\r\n").await.unwrap();
            }
            Err(_) => uart.write(b"# capture failed\r\n").await.unwrap(),
        }

        uss.power_down();
        Timer::after(Duration::from_secs(PERIOD_S)).await;
    }
}
