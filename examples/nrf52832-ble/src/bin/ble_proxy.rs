//! Internet access over BLE: advertises as "Feather-Proxy" and, once `ble_gateway.py` is connected,
//! fetches http://example.com/ every 30 seconds through it (see `proxy.rs`), logging the response
//! headers on the USB serial port at 115200. The red LED is on during a request.
//!
//! Also serves the DFU service, so the firmware can be updated over BLE.

#![no_std]
#![no_main]

#[macro_use]
#[path = "../serial.rs"]
mod serial;
#[path = "../dfu.rs"]
mod dfu;
#[path = "../proxy.rs"]
mod proxy;

use core::mem;

use dfu::{DfuService, DfuServiceEvent};
use embassy_executor::Spawner;
use embassy_futures::select::{Either3, select3};
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::interrupt::Priority;
use embassy_time::Timer;
use embedded_io_async::{Read, Write};
use nrf_softdevice::ble::advertisement_builder::{
    Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList,
};
use nrf_softdevice::ble::{PhySet, gatt_server, peripheral};
use nrf_softdevice::{Softdevice, raw};
use proxy::{Proxy, ProxyService, ProxyServiceEvent};

const DEVICE_NAME: &[u8] = b"Feather-Proxy";
const HOST: &str = "example.com";

#[nrf_softdevice::gatt_server]
struct Server {
    proxy: ProxyService,
    dfu: DfuService,
}

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

/// One HTTP/1.0 GET; logs the status line and headers, and counts the body.
async fn fetch(proxy: &Proxy<'_>) -> Result<(), proxy::Error> {
    let mut socket = proxy.connect(HOST, 80).await?;
    socket
        .write_all(b"GET / HTTP/1.0\r\nHost: example.com\r\nUser-Agent: embassy-nrf52832\r\n\r\n")
        .await?;

    let mut buf = [0u8; 256];
    // Header bytes of the current line, cut to what fits in a log line.
    let mut line: heapless::Vec<u8, 80> = heapless::Vec::new();
    let mut in_headers = true;
    let mut body = 0usize;
    loop {
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        for &b in &buf[..n] {
            if !in_headers {
                body += 1;
            } else if b == b'\n' {
                if line.is_empty() {
                    in_headers = false;
                } else {
                    log!("  {}", core::str::from_utf8(&line).unwrap_or("<binary>"));
                    line.clear();
                }
            } else if b != b'\r' {
                let _ = line.push(b);
            }
        }
    }
    log!("  ...{} bytes of body", body);
    Ok(())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Priorities 0, 1 and 4 belong to the SoftDevice.
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    spawner.spawn(serial::serial_task(p.UARTE0, p.P0_06).unwrap());
    log!("nrf52832 ble_proxy: starting SoftDevice");

    let mut led = Output::new(p.P0_17, Level::Low, OutputDrive::Standard);

    let sd_config = nrf_softdevice::Config {
        // The Feather has a 32.768 kHz crystal.
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_XTAL as u8,
            rc_ctiv: 0,
            rc_temp_ctiv: 0,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_20_PPM as u8,
        }),
        // Long connection events and a large MTU, for throughput.
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 1,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 247 }),
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
    log!("SoftDevice enabled, advertising as Feather-Proxy");

    static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .full_name("Feather-Proxy")
        .build();

    static SCAN_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .services_128(
            ServiceList::Complete,
            &[0x6e400200_b5a3_f393_e0a9_e50e24dcca9e_u128.to_le_bytes()],
        )
        .build();

    loop {
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected {
            adv_data: &ADV_DATA,
            scan_data: &SCAN_DATA,
        };
        let mut conn = match peripheral::advertise_connectable(sd, adv, &peripheral::Config::default()).await {
            Ok(conn) => conn,
            Err(e) => {
                log!("advertise error: {:?}", e);
                continue;
            }
        };
        log!("connected");
        // Best effort: the central may not support either.
        let _ = conn.phy_update(PhySet::M2, PhySet::M2);
        let _ = conn.data_length_update(None);

        let gatt = gatt_server::run(&conn, &server, |e| match e {
            ServerEvent::Proxy(e) => proxy::on_event(e),
            ServerEvent::Dfu(e) => dfu::on_event(e),
        });

        let app = async {
            let proxy = Proxy::new(&conn, &server.proxy);
            loop {
                proxy.wait_gateway().await;
                log!("GET http://{}/", HOST);
                led.set_high();
                let res = fetch(&proxy).await;
                led.set_low();
                if let Err(e) = res {
                    log!("request failed: {:?}", e);
                }
                Timer::after_secs(30).await;
            }
        };

        // `gatt` returns when the central disconnects.
        match select3(gatt, app, dfu::REQUESTED.wait()).await {
            Either3::First(e) => {
                proxy::on_disconnect();
                log!("disconnected: {:?}", e);
            }
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
