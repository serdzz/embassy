//! Turns the Feather nRF52832 into an OpenHaystack-compatible "AirTag":
//! broadcasts BLE advertisements that Apple's Find My network (any nearby
//! iPhone, no app needed on that phone) picks up and relays — you then
//! read the location back via the OpenHaystack Mac app, which decrypts
//! Find My's location reports with the matching private key.
//!
//! # How this actually works (and what you must do OUTSIDE this firmware)
//! Apple Find My tracks a rotating *public key*, not a device identity.
//! This firmware only ever broadcasts ONE static public key baked in at
//! compile time — see the "Disclaimer" in OpenHaystack's own README: a
//! device running this firmware is trackable by anyone who's recorded its
//! advertisement, because real AirTags rotate their key roughly daily and
//! this PoC does not. Good enough for "find my own stuff lying around the
//! house/garage", not good enough for anything where un-trackability
//! matters.
//!
//! You must, on a Mac, BEFORE flashing this:
//! 1. Clone <https://github.com/seemoo-lab/openhaystack> and run its
//!    `generate_keys.py` (or the OpenHaystack macOS app's "Add accessory"
//!    flow) to get one P-224 keypair. This produces a `.keys` file with a
//!    **private key** (keep on the Mac, never put in firmware) and a
//!    **Base64 advertisement key** (28 bytes decoded — this IS the public
//!    key, safe to bake into firmware).
//! 2. Base64-decode that advertisement key to 28 raw bytes, hex-encode
//!    them, and pass as the `AIRTAG_PUBKEY_HEX` env var at build time (56
//!    hex chars) — see `pubkey.rs` for the exact decode-and-validate
//!    logic and the README for the one-line shell command.
//! 3. Add the SAME key to the OpenHaystack Mac app ("load from .keys
//!    file") so it knows which Find My reports are "yours" to decrypt.
//! 4. Flash this firmware. Within a few minutes of being near ANY iPhone
//!    with Find My enabled (even a stranger's, Bluetooth-only, no app),
//!    a location report appears in the OpenHaystack Mac app.
//!
//! # BLE advertisement format (reverse-engineered by OpenHaystack/seemoo-lab,
//! not an Apple spec)
//! Manufacturer-specific data, Apple's company ID (0x004C), type 0x12
//! ("Offline Finding"), length 0x19 (25), one status byte (0x00 = normal),
//! then 22 bytes = the public key's bytes[6..28], then one more byte
//! carrying the public key's top 2 bits (`bytes[0] >> 6`) — the two most
//! significant bits of the 28-byte key don't fit in the 22-byte window, so
//! they're smuggled into this trailing byte instead. The BLE device
//! address's top 2 bits are forced to `11` (random static) and its
//! remaining 46 bits come from `bytes[0..6]` of the SAME public key — see
//! `pubkey.rs::ble_address()`. This exact layout (not anything else) is
//! what makes Find My recognise the advertisement; verified against
//! OpenHaystack's own ESP32 firmware reference implementation.
//!
//! # Low power (the whole reason this picks non-connectable advertising,
//! not the Battery+DFU GATT peripheral the other examples use)
//! A connectable peripheral (`ble_peripheral.rs`'s style) has to keep its
//! radio and SoftDevice stack ready for a central to connect at any
//! moment. This firmware never accepts connections — pure broadcast-only
//! "fire and sleep" — so the radio is off entirely between advertisement
//! bursts. `ADV_INTERVAL_MS` trades discoverability (how fast a passing
//! iPhone notices this tag) against battery life (every burst costs
//! ~1-2 mA for a few ms): 2000 ms matches real AirTags' typical interval
//! and should give low-single-digit-mA average draw, weeks-to-months on
//! a 150+ mAh LiPo — not independently measured on this exact board, so
//! treat as an estimate, not a spec.

#![no_std]
#![no_main]

#[path = "pubkey.rs"]
mod pubkey;

use embassy_executor::Spawner;
use embassy_nrf::interrupt::Priority;
use embassy_time::{Duration, Timer};
use nrf_softdevice::ble::peripheral;
use nrf_softdevice::ble::{Address, AddressType};
use nrf_softdevice::{raw, Softdevice};

use core::mem;

#[macro_use]
#[path = "../serial.rs"]
mod serial;
#[path = "../battery.rs"]
mod battery;

/// Gap between advertisement bursts — see the module doc comment's "Low
/// power" section for the tradeoff. Real AirTags use roughly this.
const ADV_INTERVAL_MS: u64 = 2000;

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

#[embassy_executor::task]
async fn battery_task(
    adc: embassy_nrf::Peri<'static, embassy_nrf::peripherals::SAADC>,
    pin: embassy_nrf::Peri<'static, embassy_nrf::peripherals::P0_31>,
) -> ! {
    battery::run(adc, pin).await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    spawner.spawn(serial::serial_task(p.UARTE0, p.P0_06).unwrap());
    log!("airtag: starting SoftDevice");
    spawner.spawn(battery_task(p.SAADC, p.P0_31).unwrap());

    let sd_config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_XTAL as u8,
            rc_ctiv: 0,
            rc_temp_ctiv: 0,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_20_PPM as u8,
        }),
        // No central/peripheral GATT role needed at all — this device
        // never connects to anything, only broadcasts. gap_role_count
        // left at Default::default() (SoftDevice's minimum) rather than
        // requesting peripheral/central roles this firmware never uses.
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: core::ptr::null_mut(),
            current_len: 0,
            max_len: 0,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&sd_config);
    spawner.spawn(softdevice_task(sd).unwrap());

    // Set this device's BLE address from the public key BEFORE the first
    // advertisement — Find My's receiving iPhone correlates the
    // advertisement payload with the sender's address, so both must be
    // derived from the same key consistently every time (see pubkey.rs).
    let addr_bytes = pubkey::ble_address();
    nrf_softdevice::ble::set_address(sd, &Address::new(AddressType::RandomStatic, addr_bytes));
    log!(
        "airtag: BLE address {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        addr_bytes[5],
        addr_bytes[4],
        addr_bytes[3],
        addr_bytes[2],
        addr_bytes[1],
        addr_bytes[0]
    );

    let adv_data = pubkey::advertisement_payload();
    log!("airtag: advertising Find My payload, {} byte(s)", adv_data.len());

    let adv_config = peripheral::Config {
        // interval is in 0.625ms units; this is independent of
        // ADV_INTERVAL_MS below — it's how fast packets repeat WITHIN one
        // advertise() call (each call below is a short single burst, not
        // a long-running advertisement), kept modest to limit on-air time
        // per burst.
        interval: 160, // 100ms
        ..Default::default()
    };

    loop {
        let adv = peripheral::NonconnectableAdvertisement::NonscannableUndirected { adv_data: &adv_data };
        // advertise() runs until the SoftDevice's own adv-duration
        // timeout fires (Config::default() uses a short bounded window)
        // or an error occurs — either way this returns, and the loop
        // just starts the next burst after sleeping. The radio is fully
        // idle during the sleep, which is where the power saving comes
        // from (see module doc comment).
        let _ = peripheral::advertise(sd, adv, &adv_config).await;
        Timer::after(Duration::from_millis(ADV_INTERVAL_MS)).await;
    }
}
