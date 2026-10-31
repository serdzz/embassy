# BLE on the Adafruit Feather nRF52832 (SoftDevice S132)

BLE examples for the Feather nRF52832 built on [`nrf-softdevice`](https://github.com/embassy-rs/nrf-softdevice),
using the S132 SoftDevice that the Adafruit bootloader already keeps in flash. Flashing works as in
[`../nrf52832`](../nrf52832/README.md): over USB through the serial bootloader, no probe needed.

| Example          | What it does                                                                 |
|------------------|------------------------------------------------------------------------------|
| `ble_peripheral` | Advertises as `Feather-Rust`, serves the Battery Service (the LiPo's charge, measured on A7 by `src/battery.rs`), an LED service (write `1`/`0` to `6e400101-b5a3-f393-e0a9-e50e24dcca9e` to switch the red LED; colours and effects for a WS2812B ring, see below), and DFU over BLE. |
| `ble_proxy`      | Advertises as `Feather-Proxy` and fetches http://example.com/ every 30 seconds through `ble_gateway.py`, which relays TCP connections for it. Also DFU over BLE. |
| `blazepod`       | BlazePod-style reaction trainer: multiple pods synchronise a round over non-connectable BLE broadcasts (no pairing/connection between pods), light their rings at the same moment, and the first button press wins — see below. |
| `airtag`         | Broadcasts an OpenHaystack-compatible Apple Find My "Offline Finding" advertisement — turns the board into a trackable-via-any-nearby-iPhone tag, decoded on a Mac with the matching private key. No GATT, no connections, broadcast-and-sleep for low power — see below. |

```sh
cargo run --release --bin ble_peripheral
```

Then connect with a BLE app such as nRF Connect. Progress is logged on the USB serial port at
115200, and so are panics.

## WS2812B (NeoPixel) ring

`ble_peripheral` drives a ring of 8 WS2812B LEDs from PWM0 on pin 7 (P0.07); `src/neopixel.rs`
generates the bit timing with PWM and EasyDMA. Wire the ring's `DI` (not `DO`) to pin 7, `GND` to
`GND`, and its supply to `3V`. At 5 V (the `USB` pin) the ring wants at least 3.5 V on `DI`, which
the nRF52's 3.3 V output does not guarantee; use a level shifter such as a 74AHCT125 there.

Two characteristics of the LED service control it:

- `6e400102-b5a3-f393-e0a9-e50e24dcca9e` takes colours: 3 bytes `R G B` for the whole ring, or 24
  bytes for one colour per LED, starting at the LED next to `DI`;
- `6e400103-b5a3-f393-e0a9-e50e24dcca9e` takes commands: `id` or `id R G B` starts an effect in
  its own or the given colour, `80 level` sets the brightness and `81 speed` the speed of the
  effects (100 is normal, 200 twice as fast).

| id   | Effect          |                                                                   |
|------|-----------------|-------------------------------------------------------------------|
| `00` | off             |                                                                   |
| `01` | solid           | the whole ring in one colour                                      |
| `02` | rainbow FTB     | a dot running round through the rainbow, its trail fading to black |
| `03` | rainbow         | the rainbow spread round the ring, turning                        |
| `04` | rainbow cycle   | the whole ring going through the rainbow                          |
| `05` | breathe         | the colour fading in and out                                      |
| `06` | comet           | a dot of the colour with a fading tail                            |
| `07` | theater chase   | every fourth LED lit, moving round                                |
| `08` | color wipe      | the ring filling with the colour and emptying again               |
| `09` | sparkle         | LEDs flashing up at random and fading                             |
| `0a` | fire            | flickering orange and red                                         |
| `0b` | police          | red and blue double flashes                                       |
| `0c` | scanner         | a dot going back and forth with a fading tail                     |
| `0d` | strobe          | short flashes of the colour                                       |

In nRF Connect, for example, write `FF0000` to the first for red, or `06 FF00FF` to the second for
a magenta comet. From a computer, `neopixel.py` does the same by name:

```sh
python3 neopixel.py fire
python3 neopixel.py brightness 255 + speed 150 + comet ff00ff
python3 neopixel.py color ff0000 ff8000 ffff00 00ff00 00ffff 0000ff 8000ff ff00ff
python3 neopixel.py demo    # every effect for 5 seconds
```

Brightness is capped at a quarter (`MAX_BRIGHTNESS`, which brightness 255 maps to), keeping the
ring's current well below what the Feather's regulator supplies.

## Internet over BLE

The board has no IP stack. `src/proxy.rs` gives it TCP sockets anyway: it asks a gateway to open a
connection to `host:port`, and the gateway relays bytes between that connection and the board over
one GATT service. The sockets implement `embedded_io_async::Read` and `Write`, so HTTP or MQTT
clients built on those traits can run on top. The frame format and the credit-based flow control
are described in `src/proxy.rs`.

```sh
cargo run --release --bin ble_proxy
python3 ble_gateway.py                        # on the machine with internet access
python3 ble_gateway.py --allow 'example.com:80'   # only let the board reach what it should
```

Without `--allow` the board can open connections to anything the gateway machine can reach,
including the local network. There is no TLS on the board (RAM is tight next to the SoftDevice);
for HTTPS, let a service on the gateway side terminate it. Outgoing connections only, at most
`MAX_SOCKETS` (2) at once, and only while the gateway is connected. On macOS, run the gateway the
same way as `dfu_ble.py` (see below).

## DFU over BLE

`ble_peripheral` exposes Nordic's legacy DFU service (`00001530-1212-efde-1523-785feabcd123`) in
application mode (`src/dfu.rs`). When a DFU client writes START_DFU to its control point, the
application sets GPREGRET to `0xA8`, the Adafruit bootloader's "OTA DFU" magic, and resets. The
bootloader comes up advertising as `AdaDFU` with the same service, and the client uploads the image
to it. This is the same handoff the Arduino core's `BLEDfu` does, except that it resets instead of
jumping into the bootloader with the SoftDevice still configured.

The package to upload is the `.zip` that `flash.sh` leaves next to the ELF, e.g.
`target/thumbv7em-none-eabihf/release/ble_peripheral.zip`. To upload it:

- from a phone: nRF Connect, nRF Toolbox (DFU) or Adafruit Bluefruit Connect; pick the zip and
  the `Feather-Rust` device;
- from a computer: `python3 dfu_ble.py target/thumbv7em-none-eabihf/release/ble_peripheral.zip`
  (needs `pip install bleak`).

  On macOS, Bluetooth access is granted to the app that launched the process, and only if that
  app's `Info.plist` declares `NSBluetoothAlwaysUsageDescription`. Terminal.app and Warp do not, so
  the script gets killed with SIGABRT even with permission granted in System Settings. Run it from
  a minimal `.app` bundle that declares the key instead: an `Info.plist` with
  `CFBundleExecutable` and `NSBluetoothAlwaysUsageDescription`, a `Contents/MacOS` script that
  `exec`s Python with its arguments, ad-hoc signed with `codesign -s -`, started with
  `open -W --stdout out.txt --stderr out.txt Runner.app --args dfu_ble.py app.zip`. macOS asks for
  Bluetooth permission on the first run.

An update of `ble_peripheral` takes about 30 seconds.

An image uploaded over BLE must keep the DFU service, or the next update has to go over USB (or
via the bootloader's own DFU mode, which it enters when there is no valid application).

## SoftDevice version

The board comes with S132 **6.1.1** (the Adafruit bootloader needs it, so it is not replaced here).
`nrf-softdevice` generates its bindings from S132 7.0.1 headers. Between the two, the API only
grew: 7.0 added three GAP calls at the end of the SVC range (connection event counter and
triggers) and two config IDs (PPCP/CAR characteristic inclusion). Call numbers and struct layouts
of everything else are unchanged, so the 7.0.1 bindings work as long as those additions are not
used. `nrf_softdevice::Config::gap_ppcp_incl` and `gap_car_incl` must stay `None`.

## Memory layout

The application sits at `0x26000` in flash, after the MBR and SoftDevice. The SoftDevice also
owns the bottom of RAM; how much depends on the configuration given to `Softdevice::enable`. The
RAM origin in `memory.x`, `0x20002c28`, is what `ble_proxy` needs with its 247-byte ATT MTU
(`ble_peripheral`, with an MTU of 128, would do with `0x20002708`). If you change the configuration
and the SoftDevice needs more, the application panics on the serial port with the address to use.

Events carrying a full 244-byte write do not fit nrf-softdevice's default event buffer, hence its
`evt-max-size-512` feature.

SoftDevice rules that apply here: interrupt priorities 0, 1 and 4 are reserved (the examples use 2
and 3), and RTC0, TIMER0, RADIO, and parts of CLOCK and POWER belong to the SoftDevice.

## BlazePod (`blazepod`)

Each pod runs `cargo run --release --bin blazepod` and needs its own ring + a tactile button
wired to P0.02 (one leg to GND, the other to P0.02; `Pull::Up` is set in software, so no external
pull-up resistor needed). Flash as many boards as you want pods — they find each other over the
air, no pairing or per-pod configuration.

**How a round works:** any pod's button press starts a round — it picks a random 0.8-3.0 s delay
and a colour, then broadcasts both (as a non-connectable BLE advertisement) so every pod in range
lights its ring at the same moment and starts its own reaction timer. First button press anywhere
broadcasts the win; every pod shows that colour, with more LEDs lit the faster the reaction, then
resets to idle after 2 seconds. A round nobody reacts to within 5 s times out back to idle.

**Why BLE broadcasts, not a BLE connection between pods:** `nrf-softdevice`'s GATT client/server is
built for one phone talking to one pod, not N pods all needing the same start signal within a few
milliseconds — see the module doc comment at the top of `src/bin/blazepod.rs` for the full
rationale (and its honest caveats: an unregistered manufacturer ID in the advertisement payload,
and "ms from the broadcaster's own clock" rather than true wall-clock sync, both fine for a
reaction game, not fine to ship as-is in a real product).

Each pod **also** still advertises connectably as `BlazePod-Rust` with the Battery Service and DFU
over BLE (same services as `ble_peripheral`), on top of the broadcast game traffic — so you can
check battery level or push a firmware update from a phone the same way, without interfering with
the game signalling.

Not yet implemented: a "spectator"/phone app that logs reaction times across rounds (the pods
currently only show results on their own LED rings and the USB serial log), and any way to
configure which pods belong to the same game session if you're running two separate groups within
radio range of each other (right now ALL pods in range join every round).

## AirTag (`airtag`)

Turns the board into an [OpenHaystack](https://github.com/seemoo-lab/openhaystack)-compatible
tracker: it broadcasts Apple's reverse-engineered "Offline Finding" BLE advertisement, which ANY
nearby iPhone with Find My enabled relays to Apple's servers — no app on that phone needed, and no
pairing between it and the board. You then read the location back on a Mac via the OpenHaystack
app, which decrypts the report with your matching private key.

**Setup is mostly OUTSIDE this repo, on a Mac, before you ever flash anything:**

1. Clone [seemoo-lab/openhaystack](https://github.com/seemoo-lab/openhaystack) and generate one
   keypair (its `generate_keys.py`, or the macOS app's "Add accessory" flow). You get a `.keys`
   file: a **private key** (keep it on the Mac — never put it in firmware) and a **Base64
   advertisement key** (this one IS the public key, safe to bake into firmware).
2. Convert that Base64 string to the hex this firmware's build expects:
   ```sh
   python3 -c "import base64,sys; print(base64.b64decode(sys.argv[1]).hex())" "<base64-key>"
   ```
3. Build and flash with that hex as an env var:
   ```sh
   AIRTAG_PUBKEY_HEX=<56 hex chars> cargo run --release --bin airtag
   ```
   (Building with `AIRTAG_PUBKEY_HEX` unset silently falls back to an all-zero key — which Find
   My won't recognise as valid, so the board just advertises harmlessly instead of tracking
   anything; a *wrong-length or non-hex* value, however, IS a compile error — see `pubkey.rs`'s
   const-eval validation. Don't skip setting it and assume it "just worked".)
4. Add the SAME `.keys` file to the OpenHaystack Mac app so it knows which Find My reports to
   decrypt as yours.
5. Wait. Within minutes of being near any iPhone with Find My on, a location report should appear
   in the OpenHaystack app.

**Honest limitations** (see the module doc comment at the top of `airtag.rs` for the full
rationale): this broadcasts ONE static public key forever — real AirTags rotate their key roughly
daily specifically so they can't be tracked long-term by a third party; this PoC firmware can be.
Fine for "find my own stuff lying around the house/garage," not fine for anything where
un-trackability matters. No GATT services at all (no Battery/DFU like the other examples) — it's
pure broadcast-and-sleep, which is also where its power saving comes from: the radio is fully idle
between ~2-second advertisement bursts, same cadence real AirTags use. Advertising interval/power
draw have NOT been independently measured on this exact board — treat the "weeks on a LiPo"
estimate in the module doc comment as a guess, not a spec.

## Toolchain

`nrf-softdevice` needs `fixed` 1.31, which requires a newer compiler than the 1.92 the repository
pins, so this crate has its own `rust-toolchain.toml` using stable.
