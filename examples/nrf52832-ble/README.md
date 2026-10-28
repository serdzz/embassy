# BLE on the Adafruit Feather nRF52832 (SoftDevice S132)

BLE examples for the Feather nRF52832 built on [`nrf-softdevice`](https://github.com/embassy-rs/nrf-softdevice),
using the S132 SoftDevice that the Adafruit bootloader already keeps in flash. Flashing works as in
[`../nrf52832`](../nrf52832/README.md): over USB through the serial bootloader, no probe needed.

| Example          | What it does                                                                 |
|------------------|------------------------------------------------------------------------------|
| `ble_peripheral` | Advertises as `Feather-Rust`, serves the Battery Service (the LiPo's charge, measured on A7 by `src/battery.rs`), an LED service (write `1`/`0` to `6e400101-b5a3-f393-e0a9-e50e24dcca9e` to switch the red LED; write colours to `6e400102-…` for a WS2812B ring, see below), and DFU over BLE. |
| `ble_proxy`      | Advertises as `Feather-Proxy` and fetches http://example.com/ every 30 seconds through `ble_gateway.py`, which relays TCP connections for it. Also DFU over BLE. |

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

Write the colours to `6e400102-b5a3-f393-e0a9-e50e24dcca9e`: 3 bytes `R G B` for the whole ring,
or 24 bytes for one colour per LED, starting at the LED next to `DI`. In nRF Connect, for example,
write `FF0000` for red. The single byte `01` starts Rainbow FTB, a dot running round the ring
through the colours of the rainbow with its trail fading to black; writing a colour stops it.
Brightness is capped at a quarter (`MAX_BRIGHTNESS`), which keeps the ring's current well below
what the Feather's regulator supplies.

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

## Toolchain

`nrf-softdevice` needs `fixed` 1.31, which requires a newer compiler than the 1.92 the repository
pins, so this crate has its own `rust-toolchain.toml` using stable.
