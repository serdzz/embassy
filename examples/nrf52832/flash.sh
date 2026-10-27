#!/bin/sh
# Cargo runner: flash an ELF onto the Adafruit Feather nRF52832 through its serial DFU bootloader,
# the same way the Arduino IDE does.
#
#   PORT  serial port of the Feather's CP2104 (default: the first /dev/cu.usbserial-*)
#
# Use the port of Apple's built-in CP210x driver (cu.usbserial-*), not the one of Silicon Labs' legacy
# VCP driver (cu.SLAB_USBtoUART): with that one the upload goes through, but the board never leaves
# the bootloader afterwards.
set -e

elf="$1"
port="${PORT:-$(ls /dev/cu.usbserial-* | head -n 1)}"
hex="$elf.hex"
zip="$elf.zip"

arm-none-eabi-objcopy -O ihex "$elf" "$hex"
# --dev-type 0x0052 is what the Arduino core passes for this board. Without --sd-req the package
# accepts any SoftDevice, so it installs whichever S132 version the bootloader came with.
adafruit-nrfutil dfu genpkg --dev-type 0x0052 --application "$hex" "$zip" >/dev/null
# nrfutil resets the board into the bootloader by toggling DTR, then flashes at the legacy 115200 baud.
adafruit-nrfutil --verbose dfu serial --package "$zip" -p "$port" -b 115200 --singlebank
