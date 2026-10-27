#!/usr/bin/env python3
"""Upload a DFU package to the Feather nRF52832 over BLE (Nordic legacy DFU, as the Adafruit
bootloader implements it).

    pip install bleak
    python3 dfu_ble.py app.zip [--name Feather-Rust]

The package is the .zip that `adafruit-nrfutil dfu genpkg` makes (flash.sh leaves one next to the
ELF). If the device is running the application, this asks it to reset into the bootloader first.
On macOS, run it from an app bundle that declares NSBluetoothAlwaysUsageDescription; see the
README. Started from Terminal.app or Warp, macOS kills it with SIGABRT.
"""

import argparse
import asyncio
import json
import struct
import sys
import zipfile

from bleak import BleakClient, BleakScanner

DFU_SERVICE = "00001530-1212-efde-1523-785feabcd123"
CONTROL = "00001531-1212-efde-1523-785feabcd123"
PACKET = "00001532-1212-efde-1523-785feabcd123"
REVISION = "00001534-1212-efde-1523-785feabcd123"

OP_START_DFU = 0x01
OP_INIT_PARAMS = 0x02
OP_RECEIVE_IMAGE = 0x03
OP_VALIDATE = 0x04
OP_ACTIVATE_AND_RESET = 0x05
OP_PKT_RCPT_NOTIF_REQ = 0x08
OP_RESPONSE = 0x10
OP_PKT_RCPT_NOTIF = 0x11

IMAGE_APPLICATION = 0x04
PACKET_SIZE = 20
# Packets between receipt notifications; the bootloader writes flash while we wait.
PRN = 8


def load_package(path):
    with zipfile.ZipFile(path) as z:
        manifest = json.loads(z.read("manifest.json"))["manifest"]
        app = manifest["application"]
        return z.read(app["bin_file"]), z.read(app["dat_file"])


async def find(name, timeout=15):
    """Find the application by name, or the bootloader by its DFU service."""

    def match(dev, adv):
        return adv.local_name in (name, "AdaDFU") or DFU_SERVICE in adv.service_uuids

    return await BleakScanner.find_device_by_filter(match, timeout=timeout)


class Dfu:
    def __init__(self, client):
        self.client = client
        self.queue = asyncio.Queue()

    async def start(self):
        await self.client.start_notify(CONTROL, lambda _, data: self.queue.put_nowait(bytes(data)))

    async def control(self, *data):
        await self.client.write_gatt_char(CONTROL, bytes(data), response=True)

    async def packet(self, data):
        await self.client.write_gatt_char(PACKET, data, response=False)

    async def expect(self, opcode, timeout=30):
        resp = await asyncio.wait_for(self.queue.get(), timeout)
        if resp[0] != OP_RESPONSE or resp[1] != opcode:
            raise RuntimeError(f"unexpected notification {resp.hex()}")
        if resp[2] != 0x01:
            raise RuntimeError(f"opcode {opcode:#x} failed with status {resp[2]:#x}")


async def in_app_mode(client):
    """The application reports DFU revision 0.1; the bootloader a higher one, or none at all."""
    try:
        rev = struct.unpack("<H", await client.read_gatt_char(REVISION))[0]
    except Exception:
        return False
    return rev == 0x0001


async def enter_bootloader(client):
    print("application mode, requesting reset into the bootloader")
    await client.start_notify(CONTROL, lambda *_: None)
    try:
        await client.write_gatt_char(CONTROL, bytes([OP_START_DFU, IMAGE_APPLICATION]), response=True)
    except Exception:
        pass  # the device may reset before the response arrives


async def upload(client, image, init):
    dfu = Dfu(client)
    await dfu.start()

    print(f"starting DFU, {len(image)} bytes")
    await dfu.control(OP_START_DFU, IMAGE_APPLICATION)
    await dfu.packet(struct.pack("<III", 0, 0, len(image)))
    await dfu.expect(OP_START_DFU)

    await dfu.control(OP_INIT_PARAMS, 0x00)
    await dfu.packet(init)
    await dfu.control(OP_INIT_PARAMS, 0x01)
    await dfu.expect(OP_INIT_PARAMS)

    await dfu.control(OP_PKT_RCPT_NOTIF_REQ, PRN & 0xFF, PRN >> 8)
    await dfu.control(OP_RECEIVE_IMAGE)

    for n, off in enumerate(range(0, len(image), PACKET_SIZE), start=1):
        await dfu.packet(image[off : off + PACKET_SIZE])
        if n % PRN == 0:
            resp = await asyncio.wait_for(dfu.queue.get(), 30)
            if resp[0] != OP_PKT_RCPT_NOTIF:
                raise RuntimeError(f"unexpected notification {resp.hex()}")
            received = struct.unpack("<I", resp[1:5])[0]
            print(f"\r{received}/{len(image)} bytes", end="", flush=True)
    print()
    await dfu.expect(OP_RECEIVE_IMAGE)

    await dfu.control(OP_VALIDATE)
    await dfu.expect(OP_VALIDATE)
    print("validated, activating")
    try:
        await dfu.control(OP_ACTIVATE_AND_RESET)
    except Exception:
        pass  # the bootloader resets right away


async def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("package")
    ap.add_argument("--name", default="Feather-Rust", help="advertised name of the application")
    args = ap.parse_args()
    image, init = load_package(args.package)

    for _ in range(2):
        dev = await find(args.name)
        if dev is None:
            sys.exit("device not found")
        async with BleakClient(dev) as client:
            if await in_app_mode(client):
                await enter_bootloader(client)
                await asyncio.sleep(2)
                continue
            await upload(client, image, init)
            print("done")
            return
    sys.exit("device did not come back in bootloader mode")


asyncio.run(main())
