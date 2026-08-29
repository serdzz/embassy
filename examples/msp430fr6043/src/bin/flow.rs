//! Ping a pair of ultrasonic transducers both ways and report the difference in flight time.
//!
//! This is the shape of a flow meter: excite one transducer, record what the other heard, swap
//! them, do it again, and compare. The difference between the two flight times is what the flow is
//! read from — a nanosecond or so at a metre a second, against a flight of tens of microseconds.
//!
//! **It has not been near a transducer.** The numbers in `Config` are guesses about somebody else's
//! plumbing, the threshold is a guess about somebody else's signal, and the estimator in
//! `uss::tof` is not TI's library. See `embassy_msp430::uss` for what that means.
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_msp430::clock::DcoFreq;
use embassy_msp430::uart::{Config as UartConfig, Uart};
use embassy_msp430::uss::{tof, Channel, Config as UssConfig, Uss};
use embassy_time::{Duration, Timer};

use panic_msp430 as _;

/// Samples per ping. Two of these fit in the buffer below, and the buffer is a fifth of the RAM
/// this device has — which is the constraint that decides how long a capture window can be.
const SAMPLES: usize = 200;

/// What counts as the burst having arrived. Wants setting from what the receiving transducer
/// actually produces on the board in question.
const THRESHOLD: i16 = 400;

/// How far the correlation searches, in samples. A little more than the largest shift the plumbing
/// can produce.
const MAX_LAG: i16 = 16;

/// How many samples after the burst to correlate. The first few cycles carry the signal; the tail
/// is mostly work and noise.
const WINDOW: usize = 64;

/// A signed decimal, up to six digits and a sign.
fn dec(value: i32, out: &mut [u8; 8]) -> &[u8] {
    out.fill(b' ');
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
            // Nothing to measure with. Say so and stop, rather than reporting zeroes that look
            // like a working meter reading no flow.
            uart.write(b"USS did not start\r\n").await.unwrap();
            loop {
                Timer::after(Duration::from_secs(60)).await;
            }
        }
    };

    let mut buf = [0i16; SAMPLES * 2];
    let mut digits = [0u8; 8];

    loop {
        // Powering down drops the analog supply and stops the crystal, so coming back is the whole
        // bring-up again, not a resume. It still pays for itself against a second of everything
        // switched off.
        if uss.restart().is_err() {
            uart.write(b"USS did not restart\r\n").await.unwrap();
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }

        match uss.capture_pair(Channel::Ch0, &mut buf).await {
            Ok((up, down)) => {
                match tof::analyse(up, down, &uss_config, THRESHOLD, MAX_LAG, WINDOW) {
                    Some(m) => {
                        uart.write(b"dt ").await.unwrap();
                        uart.write(dec(m.delta_t_ps, &mut digits)).await.unwrap();
                        uart.write(b" ps  q ").await.unwrap();
                        uart.write(dec(m.quality >> 8, &mut digits)).await.unwrap();
                        uart.write(b"\r\n").await.unwrap();
                    }
                    // No burst found, or the correlation peak ran off the end of the search. Both
                    // mean there is no answer, which is worth reporting as such.
                    None => uart.write(b"no echo\r\n").await.unwrap(),
                }
            }
            Err(_) => uart.write(b"capture failed\r\n").await.unwrap(),
        }

        // A meter measures for microseconds and sleeps for the rest of the second. The analog
        // section is the expensive part, so it goes off in between.
        uss.power_down();
        Timer::after(Duration::from_secs(1)).await;
    }
}
