#!/usr/bin/env python3
"""BLE-to-TCP gateway for the Feather nRF52832 `ble_proxy` example.

    pip install bleak
    python3 ble_gateway.py [--name Feather-Proxy] [--allow example.com:80 ...]

Connects to the board and opens the TCP connections it asks for, relaying bytes both ways. The
protocol is described in src/proxy.rs. Without --allow, the board can reach any host this machine
can, including ones on the local network; each --allow takes a host:port pattern (fnmatch style,
e.g. '*.example.com:443') and everything else is refused.

On macOS, run it from an app bundle that declares NSBluetoothAlwaysUsageDescription; see the
README. Started from Terminal.app or Warp, macOS kills it with SIGABRT.
"""

import argparse
import asyncio
import fnmatch
import struct

from bleak import BleakClient, BleakScanner

TX = "6e400201-b5a3-f393-e0a9-e50e24dcca9e"  # board -> gateway, notify
RX = "6e400202-b5a3-f393-e0a9-e50e24dcca9e"  # gateway -> board, write without response

OP_OPEN, OP_SEND, OP_CLOSE, OP_CREDIT = 0x01, 0x02, 0x03, 0x04
OP_OPENED, OP_DATA, OP_CLOSED = 0x81, 0x82, 0x83

RX_WINDOW = 1024  # matches proxy.rs: credit per socket after OPENED
STATUS_OK, STATUS_FAILED, STATUS_REFUSED = 0, 1, 2
CONNECT_TIMEOUT = 10


class Socket:
    def __init__(self):
        self.writer = None
        self.task = None
        self.credit = RX_WINDOW
        self.credit_event = asyncio.Event()
        self.credit_event.set()


class Gateway:
    def __init__(self, client, allow):
        self.client = client
        self.allow = allow
        self.sockets = {}
        self.frames = asyncio.Queue()
        # Frames to the board must not interleave.
        self.send_lock = asyncio.Lock()

    def max_payload(self):
        return min(self.client.mtu_size - 3, 244) - 2

    async def send(self, *parts):
        frame = b"".join(bytes(p) for p in parts)
        async with self.send_lock:
            await self.client.write_gatt_char(RX, frame, response=False)

    def allowed(self, host, port):
        return not self.allow or any(fnmatch.fnmatch(f"{host}:{port}", a) for a in self.allow)

    async def run(self, disconnected):
        await self.client.start_notify(TX, lambda _, data: self.frames.put_nowait(bytes(data)))
        print(f"gateway ready, MTU {self.client.mtu_size}")
        stop = asyncio.create_task(disconnected.wait())
        try:
            while True:
                get = asyncio.create_task(self.frames.get())
                done, _ = await asyncio.wait({get, stop}, return_when=asyncio.FIRST_COMPLETED)
                if stop in done:
                    get.cancel()
                    return
                await self.dispatch(get.result())
        finally:
            for sid in list(self.sockets):
                self.drop(sid)

    async def dispatch(self, frame):
        if len(frame) < 2:
            return
        op, sid, data = frame[0], frame[1], frame[2:]
        if op == OP_OPEN:
            (port,) = struct.unpack(">H", data[:2])
            host = data[2:].decode(errors="replace")
            # A reused id means the board is done with the old connection.
            self.drop(sid)
            sock = self.sockets[sid] = Socket()
            sock.task = asyncio.create_task(self.connect(sid, sock, host, port))
        elif op == OP_SEND:
            sock = self.sockets.get(sid)
            if sock and sock.writer:
                sock.writer.write(data)
                await sock.writer.drain()
        elif op == OP_CLOSE:
            print(f"[{sid}] closed by board")
            self.drop(sid)
        elif op == OP_CREDIT:
            sock = self.sockets.get(sid)
            if sock:
                sock.credit += struct.unpack(">H", data[:2])[0]
                sock.credit_event.set()

    def drop(self, sid):
        sock = self.sockets.pop(sid, None)
        if sock is None:
            return
        if sock.task:
            sock.task.cancel()
        if sock.writer:
            sock.writer.close()

    async def connect(self, sid, sock, host, port):
        if not self.allowed(host, port):
            print(f"[{sid}] refused {host}:{port}")
            await self.send([OP_OPENED, sid, STATUS_REFUSED])
            self.sockets.pop(sid, None)
            return
        try:
            reader, sock.writer = await asyncio.wait_for(asyncio.open_connection(host, port), CONNECT_TIMEOUT)
        except (OSError, asyncio.TimeoutError) as e:
            print(f"[{sid}] {host}:{port} failed: {e}")
            await self.send([OP_OPENED, sid, STATUS_FAILED])
            self.sockets.pop(sid, None)
            return
        print(f"[{sid}] connected to {host}:{port}")
        await self.send([OP_OPENED, sid, STATUS_OK])
        total = 0
        try:
            while True:
                await sock.credit_event.wait()
                data = await reader.read(min(self.max_payload(), sock.credit))
                if not data:
                    break
                sock.credit -= len(data)
                if sock.credit == 0:
                    sock.credit_event.clear()
                total += len(data)
                await self.send([OP_DATA, sid], data)
        except OSError as e:
            print(f"[{sid}] {e}")
        print(f"[{sid}] {host}:{port} closed by peer, {total} bytes relayed")
        await self.send([OP_CLOSED, sid])
        if self.sockets.get(sid) is sock:
            sock.task = None
            self.drop(sid)


async def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--name", default="Feather-Proxy", help="advertised name of the board")
    ap.add_argument("--allow", action="append", default=[], metavar="HOST:PORT")
    args = ap.parse_args()

    while True:
        print(f"scanning for {args.name}")
        dev = await BleakScanner.find_device_by_name(args.name, timeout=30)
        if dev is None:
            continue
        disconnected = asyncio.Event()
        try:
            async with BleakClient(dev, disconnected_callback=lambda _: disconnected.set()) as client:
                await Gateway(client, args.allow).run(disconnected)
        except Exception as e:  # BLE errors: just reconnect
            print(f"connection error: {e!r}")
        print("disconnected")
        await asyncio.sleep(1)


asyncio.run(main())
