# Embassy on the Adafruit Feather nRF52832

Examples for the [Adafruit Feather nRF52832](https://www.adafruit.com/product/3406), flashed over
USB through the board's serial DFU bootloader, the same way the Arduino IDE does. No debug probe is
needed.

| Example  | What it does                                                        |
|----------|---------------------------------------------------------------------|
| `blinky` | Blinks the red LED (P0.17).                                         |
| `uart`   | Prints a greeting on the USB serial port at 115200, then echoes input. |

## Memory layout

The Adafruit bootloader keeps the MBR and SoftDevice S132 at the bottom of flash and itself at the
top, so the application is linked at `0x26000` (`memory.x`). The SoftDevice is never enabled, so the
application owns all of RAM, and `cortex-m-rt`'s `set-vtor` feature points VTOR at the
application's vector table instead of relying on the SoftDevice to forward interrupts.

## Flashing

You need [`adafruit-nrfutil`](https://github.com/adafruit/Adafruit_nRF52_nrfutil) and
`arm-none-eabi-objcopy`:

```sh
pipx install adafruit-nrfutil
brew install --cask gcc-arm-embedded   # or any arm-none-eabi binutils
```

`flash.sh` is the cargo runner. It converts the ELF to a DFU package and uploads it:

```sh
cargo run --release --bin blinky
PORT=/dev/cu.usbserial-XXXXXXXX cargo run --release --bin uart
```

On macOS, use the port of Apple's built-in CP210x driver, `/dev/cu.usbserial-*`, which is the
default. If Silicon Labs' legacy VCP driver is installed, it adds `/dev/cu.SLAB_USBtoUART` for the
same chip. Uploads through that port report success, but the board then stays in the bootloader
and never starts the application.

There is no probe, so there is no `defmt` either: `uart` shows how to talk over the same serial
port instead. Opening the port resets the board (DTR), and the application starts about a second
later, after the bootloader's DFU window.
