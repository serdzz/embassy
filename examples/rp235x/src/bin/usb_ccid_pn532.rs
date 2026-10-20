//! USB CCID smart-card reader backed by a PN532 contactless (ISO14443-A)
//! reader, on RP2350 (Pico 2 / Pico 2 W / "RP2350 Plus"-style boards).
//!
//! PN532 is wired over SPI (SPIInterface, no IRQ pin — polled wait_ready).
//! Default pins below match the common Waveshare/Pimoroni RP2350 "Plus"
//! boards' SPI1 header; adjust to your wiring.
//!
//! The CCID device class itself lives in `../ccid.rs` (embassy-usb only
//! ships the PC-side CCID host class, not a device-side one).
//!
//! PC/SC (pcscd + libccid, or Windows smart card service) will enumerate
//! this as a normal USB CCID reader; tapping an ISO14443-A card/tag runs
//! InListPassiveTarget + InDataExchange (APDU pass-through) on the PN532
//! and reports ATR / APDU responses back over CCID.

#![no_std]
#![no_main]

#[path = "../ccid.rs"]
mod ccid;

use ccid::{Ccid, HostRequest, HostResponse, MAX_APDU, State as CcidState};
use defmt::{Debug2Format, info, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::spi::{self, Spi};
use embassy_rp::usb::{Driver as UsbDriver, InterruptHandler as UsbInterruptHandler};
use embassy_time::{Duration, Timer};
use panic_probe as _;
use pn532::requests::{BorrowedRequest, Command, SAMMode};
use pn532::spi::SPIInterface;
use pn532::{Pn532, Request};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
});

/// ISO14443-A historical bytes-free ATR we synthesize for a PN532 target:
/// TS=0x3B, T0=0x80 (no bytes following except TD1 implied... kept minimal),
/// this is a direct-convention ATR with no interface bytes — good enough for
/// PC/SC middleware to accept the card as present; readers don't usually
/// re-derive protocol parameters from it for a CCID short-APDU T=1 session.
const SYNTH_ATR: [u8; 4] = [0x3B, 0x80, 0x80, 0x01];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("USB CCID + PN532 contactless reader");

    let p = embassy_rp::init(Default::default());

    // --- PN532 over SPI1 ---
    // Adjust to your wiring; these match the SPI1 pins used in the `spi.rs`
    // example for RP2350 boards.
    let miso = p.PIN_12;
    let mosi = p.PIN_11;
    let clk = p.PIN_10;
    let cs_pin = p.PIN_13;

    let mut spi_config = spi::Config::default();
    spi_config.frequency = 1_000_000;
    // PN532 SPI mode is MODE_0, LSB-first (crate flips bits unless
    // `msb-spi` feature is enabled; RP2350 SPI peripheral has no native
    // LSB-first mode so we enable that feature in Cargo.toml instead).
    let spi = Spi::new_blocking(p.SPI1, clk, mosi, miso, spi_config);
    let cs = Output::new(cs_pin, Level::High);

    let interface = SPIInterface { spi, cs };
    let mut pn532: Pn532<_, _, 64> = Pn532::new_async(interface);

    // Wake the PN532 up (first SPI byte after power-on is often eaten) and
    // put it in normal reader mode.
    Timer::after(Duration::from_millis(100)).await;
    match pn532.process_async(&Request::sam_configuration(SAMMode::Normal, false), 0).await {
        Ok(_) => info!("PN532 SAMConfiguration OK"),
        Err(e) => warn!("PN532 SAMConfiguration failed: {:?}", Debug2Format(&e)),
    }

    // --- USB CCID device ---
    let driver = UsbDriver::new(p.USB, Irqs);

    let mut config = embassy_usb::Config::new(0xc0de, 0xccd0);
    config.manufacturer = Some("Embassy");
    config.product = Some("RP2350 PN532 CCID Reader");
    config.serial_number = Some("00000001");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();
    static CCID_STATE: StaticCell<CcidState> = StaticCell::new();

    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        CONFIG_DESCRIPTOR.init([0; 256]),
        BOS_DESCRIPTOR.init([0; 256]),
        &mut [],
        CONTROL_BUF.init([0; 64]),
    );

    let ccid_state = CCID_STATE.init(CcidState::new());
    let mut ccid: Ccid<'_, _> = Ccid::new(&mut builder, ccid_state, 64);

    let mut usb = builder.build();
    let usb_fut = usb.run();

    let ccid_fut = async {
        ccid.wait_connection().await;
        info!("CCID host connected");

        let mut msg_buf = [0u8; 10 + MAX_APDU];
        let mut card_present = false;
        let mut target_num: Option<u8> = None;

        loop {
            // Race: either the host sends us a CCID command, or a periodic
            // poll timer fires so we can detect card insertion/removal and
            // push RDR_to_PC_NotifySlotChange even when the host is idle.
            let poll = Timer::after(Duration::from_millis(300));
            match select(ccid.run_once(&mut msg_buf), poll).await {
                Either::First(Ok(Some(HostRequest::PowerOn))) => {
                    match poll_card(&mut pn532).await {
                        Some(num) => {
                            target_num = Some(num);
                            card_present = true;
                            info!("card powered on, target {}", num);
                            let _ = ccid.finish_request(HostResponse::Atr(&SYNTH_ATR)).await;
                        }
                        None => {
                            let _ = ccid.finish_request(HostResponse::PowerOnFailed).await;
                        }
                    }
                }
                Either::First(Ok(Some(HostRequest::PowerOff))) => {
                    if let Some(num) = target_num.take() {
                        let _ = pn532
                            .process_async(BorrowedRequest::new(Command::InRelease, &[num]), 1)
                            .await;
                    }
                    card_present = false;
                    let _ = ccid.finish_request(HostResponse::PoweredOff).await;
                }
                Either::First(Ok(Some(HostRequest::Apdu(apdu)))) => {
                    if let Some(num) = target_num {
                        let mut data = [0u8; MAX_APDU];
                        data[0] = num;
                        let apdu_len = apdu.len().min(MAX_APDU - 1);
                        data[1..1 + apdu_len].copy_from_slice(&apdu[..apdu_len]);
                        let req = BorrowedRequest::new(Command::InDataExchange, &data[..1 + apdu_len]);
                        match pn532.process_async(req, MAX_APDU).await {
                            Ok(resp) if !resp.is_empty() => {
                                // resp[0] is the InDataExchange status byte; drop it.
                                let _ = ccid.finish_request(HostResponse::ApduResponse(&resp[1..])).await;
                            }
                            _ => {
                                let _ = ccid.finish_request(HostResponse::ApduFailed).await;
                            }
                        }
                    } else {
                        let _ = ccid.finish_request(HostResponse::ApduFailed).await;
                    }
                }
                Either::First(Ok(None)) => {
                    // Short command already answered inside run_once().
                }
                Either::First(Err(e)) => {
                    warn!("CCID endpoint error: {:?}", Debug2Format(&e));
                    card_present = false;
                    target_num = None;
                    ccid.wait_connection().await;
                }
                Either::Second(_) => {
                    // Periodic presence poll: only matters while the slot is
                    // idle (host hasn't powered it on); detect insertion so
                    // we can notify, and detect removal while powered.
                    if !card_present {
                        if let Some(num) = poll_card(&mut pn532).await {
                            target_num = Some(num);
                            card_present = true;
                            let _ = ccid.notify_card(true).await;
                        }
                    }
                }
            }
        }
    };

    embassy_futures::join::join(usb_fut, ccid_fut).await;
}

/// Run InListPassiveTarget for one ISO14443-A target; return its target
/// number on success.
async fn poll_card<I, const N: usize>(pn532: &mut Pn532<I, (), N>) -> Option<u8>
where
    I: pn532::Interface,
    I::Error: core::fmt::Debug,
{
    match pn532.process_async(&Request::INLIST_ONE_ISO_A_TARGET, 32).await {
        Ok(resp) if !resp.is_empty() && resp[0] >= 1 => Some(resp[1]),
        _ => None,
    }
}
