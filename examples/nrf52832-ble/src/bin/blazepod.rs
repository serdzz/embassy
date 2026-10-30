//! "BlazePod"-style reaction trainer pod for the Adafruit Feather nRF52832.
//!
//! Each pod has a WS2812B ring (`neopixel.rs`), a tactile button, and the
//! same BLE peripheral stack (Battery Service + Nordic DFU) as
//! `ble_peripheral.rs`. Multiple pods synchronise over BLE **without a
//! connection** — see "Why broadcast, not GATT" below.
//!
//! # Game loop (per pod)
//! 1. Idle: ring off, listening for broadcasts from other pods and for its
//!    own button.
//! 2. One pod (whichever button is pressed first, or any pod designated
//!    "master" — see `ROLE` below) starts a round: picks a random delay,
//!    lights its own ring *and* broadcasts `GameMsg::RoundStart { round_id,
//!    light_at_ms }` so every pod in range lights at the same wall-clock
//!    moment (converted to "ms from now" relative to the broadcasting
//!    pod's own clock — see the clock-sync caveat below).
//! 3. Every pod (including the starter) lights its ring at `light_at_ms`
//!    and starts its own reaction timer.
//! 4. First pod whose button is hit broadcasts `GameMsg::Hit { round_id,
//!    reaction_ms }`. All pods (including the hitter) show the winner's
//!    colour and reaction time; the round ends after a short display
//!    window, back to idle.
//!
//! # Why broadcast, not GATT
//! `nrf-softdevice`'s GATT server/client model is built around point-to-
//! point connections, and this SoftDevice config caps `periph_role_count`
//! at a small number — fine for "phone configures one pod", wrong shape
//! for "N pods all need to hear the same start signal within a few
//! milliseconds of each other." BLE's non-connectable advertisement +
//! passive scan (`peripheral::advertise` / `central::scan`) is the
//! broadcast primitive actually meant for this: every pod is simultaneously
//! "the one advertising this round's state" and "listening for everyone
//! else's", with no connection setup/teardown latency and no limit on how
//! many pods can hear one broadcast. The GATT peripheral role (connectable
//! advertising) is still used **in parallel**, on a separate advertising
//! set, purely so a phone can still read battery level / push DFU the same
//! way `ble_peripheral.rs` does — see `run_gatt_peripheral` below.
//!
//! # Clock-sync caveat
//! Pods don't share a wall clock. `RoundStart::light_at_ms` is "milliseconds
//! from when the STARTER sends this broadcast", and each receiving pod
//! starts its own `light_at_ms`-length timer the instant it decodes the
//! packet — so the actual light-up skew between pods equals each pod's own
//! BLE radio+software latency to receive and parse one advertisement
//! (typically low single-digit milliseconds), not a clock drift problem.
//! Good enough for a reaction-game "who's fastest" use case; NOT a
//! microsecond-accurate multi-pod light show.

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
use core::sync::atomic::{AtomicU32, Ordering};

use dfu::{DfuService, DfuServiceEvent};
use embassy_executor::Spawner;
use embassy_futures::select::{select, select3, Either, Either3};
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::interrupt::Priority;
use embassy_nrf::{Peri, peripherals};
use embassy_time::{Duration, Instant, Timer};
use nrf_softdevice::ble::advertisement_builder::{
    Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList, ServiceUuid16,
};
use nrf_softdevice::ble::central::{self, ScanConfig};
use nrf_softdevice::ble::peripheral::{self, NonconnectableAdvertisement};
use nrf_softdevice::ble::{gatt_server};
use nrf_softdevice::{raw, Softdevice};

const DEVICE_NAME: &[u8] = b"BlazePod-Rust";

/// 3 manufacturer-specific bytes identify our game packets inside the
/// advertisement payload so we don't mistake someone's phone/headphones
/// advertisement for a round signal. Not a registered Bluetooth SIG company
/// ID — fine for a closed set of pods you built yourself, NOT fine if this
/// ships as a product (register one, or switch to a 128-bit service UUID
/// in an AD structure instead).
const GAME_MAGIC: [u8; 3] = [0xB1, 0xA2, 0xE5];

#[derive(Clone, Copy)]
enum GameMsg {
    /// `round_id` lets every pod recognise duplicates/retransmissions of
    /// the same round and ignore a stray broadcast from a previous one
    /// that's still bouncing around as a radio echo. `light_at_ms` is
    /// relative to when the STARTER pod sends this packet, not absolute —
    /// see the clock-sync doc comment at the top of this file.
    RoundStart { round_id: u8, light_at_ms: u16, color: [u8; 3] },
    /// First pod to react. `reaction_ms` is for the winner's own on-pod
    /// display and whatever a connected phone app wants to log — it is
    /// NOT used by other pods to decide who won; they just stop blinking
    /// and show the winner's colour once they hear ANY Hit for the
    /// current `round_id` (first one heard wins, by construction — radio
    /// propagation means a tied press is resolved by whoever's broadcast
    /// got there first, same as the physical game).
    Hit { round_id: u8, reaction_ms: u16 },
}

const MSG_ROUND_START: u8 = 1;
const MSG_HIT: u8 = 2;

impl GameMsg {
    /// Encodes into a buffer suitable for `adv_data`'s manufacturer-specific
    /// AD structure payload (after the 2-byte company ID placeholder slot
    /// the advertisement builder expects — see `build_adv_data`).
    fn encode(&self, buf: &mut [u8; 8]) -> usize {
        buf[0..3].copy_from_slice(&GAME_MAGIC);
        match self {
            GameMsg::RoundStart { round_id, light_at_ms, color } => {
                buf[3] = MSG_ROUND_START;
                buf[4] = *round_id;
                buf[5..7].copy_from_slice(&light_at_ms.to_le_bytes());
                buf[7] = color[0];
                // color[1..3] would need a 9th/10th byte; LegacyAdvertisementPayload
                // caps manufacturer data at what fits alongside flags+name in 31
                // bytes total, so RoundStart's colour is intentionally reduced to
                // one representative byte here — see `color_from_byte` below. A
                // real product would move to extended (not "legacy") advertising,
                // which has far more payload room, instead of shrinking the colour.
                8
            }
            GameMsg::Hit { round_id, reaction_ms } => {
                buf[3] = MSG_HIT;
                buf[4] = *round_id;
                buf[5..7].copy_from_slice(&reaction_ms.to_le_bytes());
                7
            }
        }
    }

    fn decode(buf: &[u8]) -> Option<Self> {
        if buf.len() < 5 || buf[0..3] != GAME_MAGIC {
            return None;
        }
        match buf[3] {
            MSG_ROUND_START if buf.len() >= 8 => Some(GameMsg::RoundStart {
                round_id: buf[4],
                light_at_ms: u16::from_le_bytes([buf[5], buf[6]]),
                color: color_from_byte(buf[7]),
            }),
            MSG_HIT if buf.len() >= 7 => {
                Some(GameMsg::Hit { round_id: buf[4], reaction_ms: u16::from_le_bytes([buf[5], buf[6]]) })
            }
            _ => None,
        }
    }
}

/// Expands the single transmitted colour byte back to RGB. Picks from a
/// small fixed palette rather than trying to losslessly round-trip 24-bit
/// colour through 8 bits — see the payload-size note in `GameMsg::encode`.
fn color_from_byte(b: u8) -> [u8; 3] {
    const PALETTE: [[u8; 3]; 8] = [
        [255, 0, 0],
        [0, 255, 0],
        [0, 0, 255],
        [255, 255, 0],
        [0, 255, 255],
        [255, 0, 255],
        [255, 128, 0],
        [255, 255, 255],
    ];
    PALETTE[(b as usize) % PALETTE.len()]
}

fn color_to_byte(round_id: u8) -> u8 {
    round_id % 8
}

/// Current round this pod has heard about, so it can tell a fresh
/// `RoundStart`/`Hit` apart from a stale retransmission still in flight.
/// `0xFF` is "no round yet" — a real `round_id` is `round_id as u8`
/// incrementing per round, deliberately NOT starting at 0 so the very
/// first round of a pod's lifetime doesn't collide with the idle sentinel.
static CURRENT_ROUND: AtomicU32 = AtomicU32::new(0xFFFF_FFFF);
/// Monotonic round counter this pod uses when IT starts a round. Separate
/// from CURRENT_ROUND because this pod also needs to track rounds it only
/// heard about (started by someone else).
static NEXT_ROUND_ID: AtomicU32 = AtomicU32::new(0);

#[nrf_softdevice::gatt_service(uuid = "180f")]
struct BatteryService {
    #[characteristic(uuid = "2a19", read, notify)]
    battery_level: u8,
}

#[nrf_softdevice::gatt_server]
struct Server {
    bas: BatteryService,
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

/// Debounced button read: a tactile button on P0_02 (free on the Feather —
/// see README's pinout table for what's already claimed by the radio/LED/
/// battery divider). Wire one leg to GND, the other to P0_02 with the
/// internal pull-up enabled, so idle reads HIGH and a press pulls LOW.
///
/// No EXTI/async wait-for-edge is available on this embassy-nrf version for
/// a plain `Input` (see module doc in gpio.rs), so this polls at a fixed
/// interval with simple debounce-by-confirmation: a LOW must be read twice
/// in a row, `DEBOUNCE_POLL` apart, before it counts as a press. Good
/// enough for a human-speed button; not fast enough to use as the
/// microsecond-precision reaction-time edge — see `wait_for_press` for
/// where the real reaction clock starts.
const DEBOUNCE_POLL: Duration = Duration::from_millis(3);

async fn wait_for_press(button: &Input<'_>) {
    loop {
        if button.is_low() {
            Timer::after(DEBOUNCE_POLL).await;
            if button.is_low() {
                // Wait for release before returning, so one physical press
                // can't be read as a burst of presses by the caller's loop.
                while button.is_low() {
                    Timer::after(DEBOUNCE_POLL).await;
                }
                return;
            }
        }
        Timer::after(DEBOUNCE_POLL).await;
    }
}

/// Builds the pod's current `adv_data` for one `peripheral::advertise`
/// call: flags + our manufacturer-specific `GameMsg`, no device name or
/// service UUIDs (every byte counts in a legacy 31-byte payload, and other
/// pods only care about `GAME_MAGIC`, not who we are).
fn build_adv_data(msg: &GameMsg, buf: &mut [u8; 31]) -> usize {
    // Manufacturer Specific Data AD structure: length, type 0xFF, company
    // ID placeholder (2 bytes, unregistered — see GAME_MAGIC doc comment),
    // then our payload.
    let mut payload = [0u8; 8];
    let payload_len = msg.encode(&mut payload);

    let mut i = 0;
    // Flags AD structure.
    buf[i] = 2;
    buf[i + 1] = 0x01; // AD type: Flags
    buf[i + 2] = Flag::GeneralDiscovery as u8 | Flag::LE_Only as u8;
    i += 3;

    // Manufacturer specific data AD structure.
    let mfg_len = 1 + 2 + payload_len; // type byte + 2-byte company placeholder + payload
    buf[i] = mfg_len as u8;
    buf[i + 1] = 0xFF; // AD type: Manufacturer Specific Data
    buf[i + 2] = 0xCA; // unregistered company ID placeholder, low byte
    buf[i + 3] = 0xFE; // unregistered company ID placeholder, high byte
    buf[i + 4..i + 4 + payload_len].copy_from_slice(&payload[..payload_len]);
    i += 3 + payload_len;

    i
}

/// Broadcasts one `GameMsg` a handful of times with short gaps — a single
/// non-connectable advertisement is a best-effort UDP-like broadcast, no
/// ack, so repeating it a few times cheaply trades airtime for a much
/// lower chance that every pod in range misses it due to one bad radio
/// moment. Not a reliability GUARANTEE — see the module doc's clock-sync
/// caveat for why that's an acceptable trade-off here.
async fn broadcast(sd: &Softdevice, msg: GameMsg) {
    let mut buf = [0u8; 31];
    let len = build_adv_data(&msg, &mut buf);
    let adv_data = &buf[..len];

    for _ in 0..3 {
        let adv = NonconnectableAdvertisement::NonscannableUndirected { adv_data };
        let config = peripheral::Config {
            interval: 20, // 20 * 0.625ms = 12.5ms between packets within one advertise() call
            ..Default::default()
        };
        // advertise() for non-connectable ads runs until its own internal
        // timeout/terminated event, which depends on `Config` — rather than
        // depend on that, race it against a short deadline per repeat so
        // this function returns promptly regardless of SoftDevice's own
        // advertising-duration bookkeeping.
        let _ = select(peripheral::advertise(sd, adv, &config), Timer::after_millis(30)).await;
    }
}

/// Listens for `GameMsg` broadcasts from other pods, forever, calling `on_msg`
/// for each one successfully decoded. Runs as its own task so the main game
/// loop (button polling, lighting the ring, deciding round outcomes) isn't
/// blocked waiting on the radio — scanning and the rest of the game state
/// machine communicate purely through the `CURRENT_ROUND`/`NEXT_ROUND_ID`
/// atomics and the two `Signal`s below, no shared mutable game state.
#[embassy_executor::task]
async fn scan_task(sd: &'static Softdevice) -> ! {
    loop {
        let config = ScanConfig {
            active: false,
            interval: 40,  // 25ms, well under the 12.5ms advertise interval so we don't miss gaps
            window: 40,
            timeout: 0, // scan forever; broken out of only by the closure below (never, here)
            ..Default::default()
        };
        let result: Result<(), _> = central::scan(sd, &config, |report| {
            let data = unsafe {
                core::slice::from_raw_parts(report.data.p_data, report.data.len as usize)
            };
            if let Some(msg) = find_game_msg(data) {
                handle_incoming(msg);
            }
            None::<()> // keep scanning
        })
        .await;
        if let Err(e) = result {
            log!("scan error, restarting: {:?}", e);
            Timer::after_millis(50).await;
        }
    }
}

/// Walks the AD structures in a raw advertisement payload looking for our
/// Manufacturer Specific Data structure, and decodes it if found. Ignores
/// everything else (flags, name, other manufacturers' data) rather than
/// requiring an exact-match payload shape — broadcasts share the air with
/// phones, headphones, etc.
fn find_game_msg(data: &[u8]) -> Option<GameMsg> {
    let mut i = 0;
    while i + 1 < data.len() {
        let len = data[i] as usize;
        if len == 0 || i + 1 + len > data.len() {
            break;
        }
        let ad_type = data[i + 1];
        let ad_payload = &data[i + 2..i + 1 + len];
        if ad_type == 0xFF && ad_payload.len() >= 2 {
            // Skip the 2-byte company ID placeholder, decode the rest.
            if let Some(msg) = GameMsg::decode(&ad_payload[2..]) {
                return Some(msg);
            }
        }
        i += 1 + len;
    }
    None
}

/// Round outcome signalled from `scan_task`/the button handler to the main
/// game loop. `Signal` (not a `Channel`) because only the latest state
/// matters — if two Hits for the same round arrive before the main loop
/// checks, only the first one mattered anyway (see `GameMsg::Hit` doc
/// comment on tie-breaking).
static ROUND_START_SIGNAL: embassy_sync::signal::Signal<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    (u8, u16, [u8; 3]),
> = embassy_sync::signal::Signal::new();
static HIT_SIGNAL: embassy_sync::signal::Signal<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    (u8, u16),
> = embassy_sync::signal::Signal::new();

fn handle_incoming(msg: GameMsg) {
    match msg {
        GameMsg::RoundStart { round_id, light_at_ms, color } => {
            let current = CURRENT_ROUND.load(Ordering::Relaxed);
            if current == round_id as u32 {
                return; // duplicate retransmission of a round we've already started timing
            }
            CURRENT_ROUND.store(round_id as u32, Ordering::Relaxed);
            ROUND_START_SIGNAL.signal((round_id, light_at_ms, color));
        }
        GameMsg::Hit { round_id, reaction_ms } => {
            // Only the first Hit heard for the current round matters — see
            // the tie-breaking note on GameMsg::Hit.
            if round_id as u32 == CURRENT_ROUND.load(Ordering::Relaxed) {
                HIT_SIGNAL.signal((round_id, reaction_ms));
            }
        }
    }
}

/// Runs the connectable BLE peripheral role (Battery Service + DFU) in
/// parallel with the broadcast game traffic — see "Why broadcast, not
/// GATT" in the module doc comment for why these are two separate
/// advertising roles instead of folding the game into GATT characteristics.
#[embassy_executor::task]
async fn gatt_peripheral_task(sd: &'static Softdevice, server: &'static Server) {
    static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .services_16(ServiceList::Incomplete, &[ServiceUuid16::BATTERY])
        .full_name("BlazePod-Rust")
        .build();
    static SCAN_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new().build();

    loop {
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected {
            adv_data: &ADV_DATA,
            scan_data: &SCAN_DATA,
        };
        let conn = match peripheral::advertise_connectable(sd, adv, &peripheral::Config::default()).await {
            Ok(conn) => conn,
            Err(e) => {
                log!("gatt advertise error: {:?}", e);
                continue;
            }
        };
        log!("phone connected");
        let _ = server.bas.battery_level_set(&battery::PERCENT.load(Ordering::Relaxed));

        let gatt = gatt_server::run(&conn, server, |e| match e {
            ServerEvent::Bas(BatteryServiceEvent::BatteryLevelCccdWrite { .. }) => {}
            ServerEvent::Dfu(e) => dfu::on_event(e),
        });
        match select(gatt, dfu::REQUESTED.wait()).await {
            Either::First(e) => log!("phone disconnected: {:?}", e),
            Either::Second(()) => {
                log!("DFU requested, resetting into the bootloader");
                Timer::after_millis(100).await;
                dfu::enter_bootloader();
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    spawner.spawn(serial::serial_task(p.UARTE0, p.P0_06).unwrap());
    log!("blazepod: starting SoftDevice");

    let mut status_led = Output::new(p.P0_17, Level::Low, OutputDrive::Standard);
    spawner.spawn(neopixel_task(p.PWM0, p.P0_07).unwrap());
    spawner.spawn(battery_measure_task(p.SAADC, p.P0_31).unwrap());

    // Button: P0_02, internal pull-up, idle HIGH / pressed LOW — see the
    // wiring note on `wait_for_press`.
    let button = Input::new(p.P0_02, Pull::Up);

    let sd_config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_XTAL as u8,
            rc_ctiv: 0,
            rc_temp_ctiv: 0,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_20_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t { conn_count: 1, event_length: 24 }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 128 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 1,
            // Central role needed for `central::scan` to listen to other
            // pods' broadcasts, even though this pod never *connects* as a
            // central — scanning still consumes this role slot.
            central_role_count: 1,
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
    static SERVER: static_cell::StaticCell<Server> = static_cell::StaticCell::new();
    let server = SERVER.init(Server::new(sd).unwrap());
    spawner.spawn(softdevice_task(sd).unwrap());
    spawner.spawn(scan_task(sd).unwrap());
    spawner.spawn(gatt_peripheral_task(sd, server).unwrap());
    log!("SoftDevice enabled, BlazePod ready — press the button to start a round");

    loop {
        // Idle: race "my own button pressed" against "someone else started
        // a round" — whichever happens first decides this pod's role for
        // the upcoming round.
        let outcome = select(wait_for_press(&button), ROUND_START_SIGNAL.wait()).await;

        let (round_id, light_at_ms, color) = match outcome {
            Either::First(()) => {
                // We're starting this round.
                let round_id = (NEXT_ROUND_ID.fetch_add(1, Ordering::Relaxed) % 255) as u8 + 1;
                CURRENT_ROUND.store(round_id as u32, Ordering::Relaxed);
                let light_at_ms: u16 = 800 + (round_id as u16 * 37) % 2200; // pseudo-random-ish 0.8-3.0s
                let color = color_from_byte(color_to_byte(round_id));
                log!("round {} starting, light in {} ms", round_id, light_at_ms);
                broadcast(sd, GameMsg::RoundStart { round_id, light_at_ms, color }).await;
                (round_id, light_at_ms, color)
            }
            Either::Second((round_id, light_at_ms, color)) => {
                log!("round {} heard, light in {} ms", round_id, light_at_ms);
                (round_id, light_at_ms, color)
            }
        };

        // Wait out the delay, racing against a Hit arriving early (e.g.
        // another pod's button got mashed before anyone's ring lit — treat
        // that as a false start: skip this pod's own light-up/reaction
        // window and go straight to showing the result).
        let pre_light = select(Timer::after_millis(light_at_ms as u64), HIT_SIGNAL.wait()).await;
        let mut already_hit: Option<(u8, u16)> = None;
        if let Either::Second(hit) = pre_light {
            already_hit = Some(hit);
        } else {
            neopixel::COMMANDS.send(neopixel::Command::Colors([color; neopixel::NUM_LEDS])).await;
            status_led.set_high();
        }

        let reaction_start = Instant::now();
        let hit = if let Some(h) = already_hit {
            h
        } else {
            // Race this pod's own button against hearing another pod's Hit
            // broadcast, for up to 5 seconds — a round that nobody reacts
            // to within that window (e.g. everyone walked away) just times
            // out back to idle rather than hanging forever.
            match select3(wait_for_press(&button), HIT_SIGNAL.wait(), Timer::after_secs(5)).await {
                Either3::First(()) => {
                    let reaction_ms = reaction_start.elapsed().as_millis().min(u16::MAX as u64) as u16;
                    log!("hit! round {} reaction {} ms", round_id, reaction_ms);
                    broadcast(sd, GameMsg::Hit { round_id, reaction_ms }).await;
                    (round_id, reaction_ms)
                }
                Either3::Second(h) => h,
                Either3::Third(()) => {
                    log!("round {} timed out, nobody hit", round_id);
                    status_led.set_low();
                    neopixel::COMMANDS
                        .send(neopixel::Command::Colors([[0, 0, 0]; neopixel::NUM_LEDS]))
                        .await;
                    CURRENT_ROUND.store(0xFFFF_FFFF, Ordering::Relaxed);
                    continue;
                }
            }
        };

        status_led.set_low();
        // Show the winner's reaction time as a simple heuristic: fewer lit
        // LEDs = slower. A real product would send this to a phone app
        // instead of trying to cram it into 8 pixels.
        let lit = (8usize.saturating_sub(hit.1 as usize / 100)).clamp(1, 8);
        let mut display = [[0u8, 0, 0]; neopixel::NUM_LEDS];
        for px in display.iter_mut().take(lit) {
            *px = color;
        }
        neopixel::COMMANDS.send(neopixel::Command::Colors(display)).await;
        Timer::after_secs(2).await;
        neopixel::COMMANDS.send(neopixel::Command::Colors([[0, 0, 0]; neopixel::NUM_LEDS])).await;
        CURRENT_ROUND.store(0xFFFF_FFFF, Ordering::Relaxed);
    }
}
