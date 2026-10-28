//! BLE peripheral on the SoftDevice: advertises as "Feather-Rust" and serves
//!
//! - the Battery Service (0x180F), with the charge of the LiPo measured on A7 (see `battery.rs`),
//!   notified when it changes;
//! - an LED service: write 1/0 to its first characteristic to turn the red LED on/off, and colours
//!   to its second to light a ring of 8 WS2812B LEDs whose data input is on pin 7 (P0.07): 3 bytes
//!   (R, G, B) for the whole ring, 24 for one colour per LED, or the single byte 1 for the Rainbow
//!   FTB effect (see `neopixel.rs`);
//! - Nordic's legacy DFU service, to update the firmware over BLE (see `dfu.rs`).
//!
//! Progress is logged on the USB serial port at 115200.

#![no_std]
#![no_main]

#[macro_use]
#[path = "../serial.rs"]
mod serial;
#[path = "../battery.rs"]
mod battery;
#[path = "../dfu.rs"]
mod dfu;
#[path = "../neopixel.rs"]
mod neopixel;

use core::mem;
use core::sync::atomic::Ordering;

use dfu::{DfuService, DfuServiceEvent};
use embassy_executor::Spawner;
use embassy_futures::select::{Either3, select3};
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::interrupt::Priority;
use embassy_nrf::{Peri, peripherals};
use embassy_time::Timer;
use heapless::Vec;
use nrf_softdevice::ble::advertisement_builder::{
    Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList, ServiceUuid16,
};
use nrf_softdevice::ble::{Connection, gatt_server, peripheral};
use nrf_softdevice::{Softdevice, raw};

const DEVICE_NAME: &[u8] = b"Feather-Rust";

#[nrf_softdevice::gatt_service(uuid = "180f")]
struct BatteryService {
    #[characteristic(uuid = "2a19", read, notify)]
    battery_level: u8,
}

#[nrf_softdevice::gatt_service(uuid = "6e400100-b5a3-f393-e0a9-e50e24dcca9e")]
struct LedService {
    #[characteristic(uuid = "6e400101-b5a3-f393-e0a9-e50e24dcca9e", read, write)]
    led: u8,
    #[characteristic(uuid = "6e400102-b5a3-f393-e0a9-e50e24dcca9e", write)]
    pixels: Vec<u8, { 3 * neopixel::NUM_LEDS }>,
}

#[nrf_softdevice::gatt_server]
struct Server {
    bas: BatteryService,
    led: LedService,
    dfu: DfuService,
}

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

#[embassy_executor::task]
async fn neopixel_task(pwm: Peri<'static, peripherals::PWM0>, pin: Peri<'static, peripherals::P0_07>) -> ! {
    neopixel::run(pwm, pin).await
}

#[embassy_executor::task]
async fn battery_measure_task(adc: Peri<'static, peripherals::SAADC>, pin: Peri<'static, peripherals::P0_31>) -> ! {
    battery::run(adc, pin).await
}

/// Notifies the battery level when it changes, for as long as the connection lasts.
async fn battery_task(server: &Server, conn: &Connection) {
    let mut sent = server.bas.battery_level_get().unwrap_or(0);
    loop {
        Timer::after_secs(1).await;
        let level = battery::PERCENT.load(Ordering::Relaxed);
        if level != sent {
            sent = level;
            // Errors here just mean the central has not enabled notifications; reads still see
            // the new value.
            if server.bas.battery_level_notify(conn, &level).is_err() {
                let _ = server.bas.battery_level_set(&level);
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Priorities 0, 1 and 4 belong to the SoftDevice.
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    spawner.spawn(serial::serial_task(p.UARTE0, p.P0_06).unwrap());
    log!("nrf52832-ble: starting SoftDevice");

    let mut led = Output::new(p.P0_17, Level::Low, OutputDrive::Standard);
    spawner.spawn(neopixel_task(p.PWM0, p.P0_07).unwrap());
    spawner.spawn(battery_measure_task(p.SAADC, p.P0_31).unwrap());

    let sd_config = nrf_softdevice::Config {
        // The Feather has a 32.768 kHz crystal.
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_XTAL as u8,
            rc_ctiv: 0,
            rc_temp_ctiv: 0,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_20_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 1,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 128 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 1,
            central_role_count: 0,
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: DEVICE_NAME.as_ptr() as _,
            current_len: DEVICE_NAME.len() as u16,
            max_len: DEVICE_NAME.len() as u16,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&sd_config);
    let server = Server::new(sd).unwrap();
    spawner.spawn(softdevice_task(sd).unwrap());
    log!("SoftDevice enabled, advertising as Feather-Rust");

    static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .services_16(ServiceList::Incomplete, &[ServiceUuid16::BATTERY])
        .full_name("Feather-Rust")
        .build();

    static SCAN_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .services_128(
            ServiceList::Complete,
            &[0x6e400100_b5a3_f393_e0a9_e50e24dcca9e_u128.to_le_bytes()],
        )
        .build();

    loop {
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected {
            adv_data: &ADV_DATA,
            scan_data: &SCAN_DATA,
        };
        let conn = match peripheral::advertise_connectable(sd, adv, &peripheral::Config::default()).await {
            Ok(conn) => conn,
            Err(e) => {
                log!("advertise error: {:?}", e);
                continue;
            }
        };
        log!("connected");
        let _ = server.bas.battery_level_set(&battery::PERCENT.load(Ordering::Relaxed));

        let gatt = gatt_server::run(&conn, &server, |e| match e {
            ServerEvent::Bas(BatteryServiceEvent::BatteryLevelCccdWrite { notifications }) => {
                log!("battery notifications: {}", notifications);
            }
            ServerEvent::Led(LedServiceEvent::LedWrite(v)) => {
                log!("led: {}", v);
                led.set_level(if v != 0 { Level::High } else { Level::Low });
            }
            ServerEvent::Led(LedServiceEvent::PixelsWrite(v)) => {
                if neopixel::command_from_bytes(&v) {
                    // 24 bytes would not fit a log line.
                    log!(
                        "pixels: {:02x?}{}",
                        &v[..v.len().min(3)],
                        if v.len() > 3 { " ..." } else { "" }
                    );
                } else {
                    log!("pixels: {} bytes, expected 1, 3 or {}", v.len(), 3 * neopixel::NUM_LEDS);
                }
            }
            ServerEvent::Dfu(e) => dfu::on_event(e),
        });

        // `gatt` returns when the central disconnects.
        match select3(gatt, battery_task(&server, &conn), dfu::REQUESTED.wait()).await {
            Either3::First(e) => log!("disconnected: {:?}", e),
            Either3::Second(_) => unreachable!(),
            Either3::Third(()) => {
                log!("DFU requested, resetting into the bootloader");
                // Let the write response and the log line go out first.
                Timer::after_millis(100).await;
                dfu::enter_bootloader();
            }
        }
    }
}
