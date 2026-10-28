#!/usr/bin/env python3
"""Control the WS2812B ring of `ble_peripheral` over BLE.

    python3 neopixel.py rainbow                 # start an effect in its own colours
    python3 neopixel.py comet ff00ff            # ... or in the given colour
    python3 neopixel.py color 00ff00            # the whole ring in one colour
    python3 neopixel.py color ff0000 00ff00 ... # one colour per LED (8 of them)
    python3 neopixel.py brightness 128          # 0..255, 255 is the firmware's cap
    python3 neopixel.py speed 200               # 100 is normal, 200 twice as fast
    python3 neopixel.py demo                    # every effect for a few seconds
    python3 neopixel.py list

Several commands can go in one run, separated by `+`: `brightness 255 + speed 150 + fire`.
Needs `pip install bleak`; on macOS run it from an app bundle, as described in the README.
"""

import argparse
import asyncio
import sys

from bleak import BleakClient, BleakScanner

PIXELS = "6e400102-b5a3-f393-e0a9-e50e24dcca9e"
CONTROL = "6e400103-b5a3-f393-e0a9-e50e24dcca9e"
NUM_LEDS = 8

# The ids of `neopixel::Effect`.
EFFECTS = [
    "off", "solid", "rainbow-ftb", "rainbow", "rainbow-cycle", "breathe", "comet",
    "theater-chase", "color-wipe", "sparkle", "fire", "police", "scanner", "strobe",
]


def rgb(text):
    value = bytes.fromhex(text.lstrip("#"))
    if len(value) != 3:
        raise ValueError(f"{text!r} is not a colour like ff8000")
    return value


def parse(words):
    """Turns one command into (characteristic, payload) writes."""
    name, args = words[0], words[1:]
    if name == "color":
        if len(args) not in (1, NUM_LEDS):
            raise ValueError(f"color takes 1 or {NUM_LEDS} colours")
        return [(PIXELS, b"".join(rgb(a) for a in args))]
    if name == "brightness":
        return [(CONTROL, bytes([0x80, int(args[0])]))]
    if name == "speed":
        return [(CONTROL, bytes([0x81, int(args[0])]))]
    if name in EFFECTS:
        payload = bytes([EFFECTS.index(name)])
        if args:
            payload += rgb(args[0])
        return [(CONTROL, payload)]
    raise ValueError(f"unknown command {name!r}; try `list`")


async def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--name", default="Feather-Rust", help="advertised name of the board")
    ap.add_argument("--seconds", type=float, default=5, help="time per effect in `demo`")
    ap.add_argument("command", nargs="+")
    args = ap.parse_args()

    if args.command == ["list"]:
        print("effects:", ", ".join(EFFECTS))
        print("other:   color RRGGBB [x8], brightness 0..255, speed 1..255, demo")
        return

    demo = args.command == ["demo"]
    steps = []
    if not demo:
        command = []
        for word in args.command + ["+"]:
            if word == "+":
                if command:
                    steps += parse(command)
                command = []
            else:
                command.append(word)

    device = await BleakScanner.find_device_by_name(args.name, timeout=20)
    if device is None:
        sys.exit(f"{args.name} not found")
    async with BleakClient(device) as client:
        if demo:
            for name in EFFECTS[2:]:
                print(name, flush=True)
                await client.write_gatt_char(CONTROL, bytes([EFFECTS.index(name)]), response=True)
                await asyncio.sleep(args.seconds)
            return
        for uuid, payload in steps:
            await client.write_gatt_char(uuid, payload, response=True)
        print("ok", flush=True)


if __name__ == "__main__":
    asyncio.run(main())
