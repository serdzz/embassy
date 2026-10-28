//! TCP sockets over BLE, through a gateway (`ble_gateway.py`) that opens the real connections.
//!
//! The board has no IP stack. It asks the gateway to open a TCP connection to `host:port`, and the
//! gateway relays bytes between that connection and the board. Frames travel over two
//! characteristics of one GATT service, one frame per ATT packet:
//!
//! | board → gateway (notify `tx`) | gateway → board (write `rx`) |
//! |-------------------------------|------------------------------|
//! | `01 id port:u16be host…` OPEN | `81 id status` OPENED (0 = ok) |
//! | `02 id data…` SEND            | `82 id data…` DATA            |
//! | `03 id` CLOSE                 | `83 id` CLOSED (peer closed or failed) |
//! | `04 id n:u16be` CREDIT        |                               |
//!
//! Gateway → board traffic is credit based: after OPENED the gateway may send `RX_WINDOW` bytes of
//! DATA for the socket, and the board hands back credit as the application reads, so the receive
//! buffer can never overflow.
//!
//! The gateway is ready once it subscribes to `tx`. Subscribing again, or a disconnect, resets
//! every socket.

use core::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

use embassy_futures::select::{Either, select};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::pipe::Pipe;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer, with_timeout};
use heapless::Vec;
use nrf_softdevice::RawError;
use nrf_softdevice::ble::Connection;
use nrf_softdevice::ble::gatt_server::NotifyValueError;

pub const MAX_SOCKETS: usize = 2;
/// Receive buffer per socket, and the credit the gateway starts with.
const RX_WINDOW: usize = 1024;
/// Hand credit back once this much has been read, so CREDIT frames stay rare.
const CREDIT_BATCH: usize = RX_WINDOW / 4;
/// Largest frame: an ATT MTU of 247 minus the 3-byte ATT header.
const FRAME_MAX: usize = 244;
const HEADER: usize = 2;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);

const OP_OPEN: u8 = 0x01;
const OP_SEND: u8 = 0x02;
const OP_CLOSE: u8 = 0x03;
const OP_CREDIT: u8 = 0x04;
const OP_OPENED: u8 = 0x81;
const OP_DATA: u8 = 0x82;
const OP_CLOSED: u8 = 0x83;

#[nrf_softdevice::gatt_service(uuid = "6e400200-b5a3-f393-e0a9-e50e24dcca9e")]
pub struct ProxyService {
    #[characteristic(uuid = "6e400201-b5a3-f393-e0a9-e50e24dcca9e", notify)]
    pub tx: Vec<u8, FRAME_MAX>,
    #[characteristic(uuid = "6e400202-b5a3-f393-e0a9-e50e24dcca9e", write_without_response)]
    pub rx: Vec<u8, FRAME_MAX>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The BLE connection to the gateway is gone.
    Disconnected,
    /// The gateway did not subscribe in time.
    NoGateway,
    /// All `MAX_SOCKETS` sockets are in use.
    NoFreeSocket,
    /// The gateway could not open the connection; the status it reported.
    ConnectFailed(u8),
    Timeout,
    /// The host name does not fit in one frame.
    HostTooLong,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl core::error::Error for Error {}

impl embedded_io_async::Error for Error {
    fn kind(&self) -> embedded_io_async::ErrorKind {
        use embedded_io_async::ErrorKind;
        match self {
            Error::Disconnected => ErrorKind::NotConnected,
            Error::NoGateway => ErrorKind::NotConnected,
            Error::NoFreeSocket => ErrorKind::OutOfMemory,
            Error::ConnectFailed(_) => ErrorKind::ConnectionRefused,
            Error::Timeout => ErrorKind::TimedOut,
            Error::HostTooLong => ErrorKind::InvalidInput,
        }
    }
}

const FREE: u8 = 0;
/// OPEN sent; DATA is ignored until OPENED, so frames left over from a previous use of the id
/// cannot leak into this one.
const OPENING: u8 = 1;
const OPEN: u8 = 2;
/// The gateway sent CLOSED; buffered data can still be read.
const CLOSED: u8 = 3;

struct Slot {
    state: AtomicU8,
    rx: Pipe<CriticalSectionRawMutex, RX_WINDOW>,
    /// Status from OPENED, or `u8::MAX` for CLOSED/reset while opening.
    opened: Signal<CriticalSectionRawMutex, u8>,
    /// Raised on CLOSED or reset, to wake a reader waiting on an empty buffer.
    closed: Signal<CriticalSectionRawMutex, ()>,
    /// Bytes read since the last CREDIT.
    consumed: AtomicUsize,
}

impl Slot {
    const fn new() -> Self {
        Self {
            state: AtomicU8::new(FREE),
            rx: Pipe::new(),
            opened: Signal::new(),
            closed: Signal::new(),
            consumed: AtomicUsize::new(0),
        }
    }

    fn close(&self) {
        match self.state.load(Ordering::Acquire) {
            OPENING => {
                self.state.store(CLOSED, Ordering::Release);
                self.opened.signal(u8::MAX);
            }
            OPEN => {
                self.state.store(CLOSED, Ordering::Release);
                self.closed.signal(());
            }
            _ => {}
        }
    }
}

static SLOTS: [Slot; MAX_SOCKETS] = [const { Slot::new() }; MAX_SOCKETS];
static GATEWAY_READY: AtomicBool = AtomicBool::new(false);
static GATEWAY_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Call from the GATT server's event handler.
pub fn on_event(event: ProxyServiceEvent) {
    match event {
        ProxyServiceEvent::TxCccdWrite { notifications } => {
            // A (re)subscribing gateway knows nothing of earlier sockets.
            reset();
            GATEWAY_READY.store(notifications, Ordering::Release);
            if notifications {
                GATEWAY_SIGNAL.signal(());
            }
        }
        ProxyServiceEvent::RxWrite(frame) => on_frame(&frame),
    }
}

/// Call when the BLE connection ends.
pub fn on_disconnect() {
    GATEWAY_READY.store(false, Ordering::Release);
    reset();
}

fn reset() {
    for slot in &SLOTS {
        slot.close();
    }
}

fn on_frame(frame: &[u8]) {
    let (&[op, id], data) = frame.split_at(HEADER.min(frame.len())) else {
        return;
    };
    let Some(slot) = SLOTS.get(id as usize) else {
        return;
    };
    match op {
        OP_OPENED if slot.state.load(Ordering::Acquire) == OPENING => {
            let status = data.first().copied().unwrap_or(u8::MAX);
            slot.state
                .store(if status == 0 { OPEN } else { CLOSED }, Ordering::Release);
            slot.opened.signal(status);
        }
        OP_DATA if slot.state.load(Ordering::Acquire) == OPEN => {
            // `try_write` stops at the end of the ring buffer, so it may take two calls. The buffer
            // cannot fill up: the gateway never sends more than the credit it was given.
            let mut data = data;
            while let Ok(n) = slot.rx.try_write(data) {
                data = &data[n..];
                if data.is_empty() {
                    break;
                }
            }
        }
        OP_CLOSED => slot.close(),
        _ => {}
    }
}

/// Opens sockets through the gateway connected on `conn`.
pub struct Proxy<'a> {
    conn: &'a Connection,
    service: &'a ProxyService,
}

impl<'a> Proxy<'a> {
    pub fn new(conn: &'a Connection, service: &'a ProxyService) -> Self {
        Self { conn, service }
    }

    /// Waits until a gateway has subscribed.
    pub async fn wait_gateway(&self) {
        while !GATEWAY_READY.load(Ordering::Acquire) {
            GATEWAY_SIGNAL.wait().await;
        }
    }

    pub async fn connect(&self, host: &str, port: u16) -> Result<TcpSocket<'_, 'a>, Error> {
        if !GATEWAY_READY.load(Ordering::Acquire) {
            return Err(Error::NoGateway);
        }
        let id = SLOTS
            .iter()
            .position(|s| {
                s.state
                    .compare_exchange(FREE, OPENING, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
            })
            .ok_or(Error::NoFreeSocket)?;
        let slot = &SLOTS[id];
        // From here on, dropping the socket frees the slot.
        let socket = TcpSocket {
            proxy: self,
            id: id as u8,
        };
        slot.rx.clear();
        slot.opened.reset();
        slot.closed.reset();
        slot.consumed.store(0, Ordering::Relaxed);

        let mut frame: Vec<u8, FRAME_MAX> = Vec::new();
        let _ = frame.extend_from_slice(&[OP_OPEN, id as u8]);
        let _ = frame.extend_from_slice(&port.to_be_bytes());
        frame
            .extend_from_slice(host.as_bytes())
            .map_err(|_| Error::HostTooLong)?;
        self.send(&frame).await?;

        match with_timeout(CONNECT_TIMEOUT, slot.opened.wait()).await {
            Ok(0) => Ok(socket),
            Ok(u8::MAX) => Err(Error::Disconnected),
            Ok(status) => Err(Error::ConnectFailed(status)),
            Err(_) => Err(Error::Timeout),
        }
    }

    /// Largest payload per frame on this connection.
    fn max_payload(&self) -> usize {
        (self.conn.att_mtu() as usize - 3).min(FRAME_MAX) - HEADER
    }

    fn try_send(&self, frame: &[u8]) -> Result<bool, Error> {
        let frame = Vec::from_slice(frame).unwrap();
        match self.service.tx_notify(self.conn, &frame) {
            Ok(()) => Ok(true),
            // The SoftDevice's notification queue is full.
            Err(NotifyValueError::Raw(RawError::Resources)) => Ok(false),
            Err(_) => Err(Error::Disconnected),
        }
    }

    async fn send(&self, frame: &[u8]) -> Result<(), Error> {
        while !self.try_send(frame)? {
            Timer::after_millis(5).await;
        }
        Ok(())
    }
}

/// A TCP connection opened by the gateway. Closed (best effort) when dropped.
pub struct TcpSocket<'p, 'a> {
    proxy: &'p Proxy<'a>,
    id: u8,
}

impl TcpSocket<'_, '_> {
    fn slot(&self) -> &'static Slot {
        &SLOTS[self.id as usize]
    }

    /// Tells the gateway how much more it may send once enough has been read.
    async fn return_credit(&self, n: usize) -> Result<(), Error> {
        let slot = self.slot();
        let consumed = slot.consumed.fetch_add(n, Ordering::Relaxed) + n;
        if consumed < CREDIT_BATCH || slot.state.load(Ordering::Acquire) != OPEN {
            return Ok(());
        }
        slot.consumed.store(0, Ordering::Relaxed);
        let [hi, lo] = (consumed as u16).to_be_bytes();
        self.proxy.send(&[OP_CREDIT, self.id, hi, lo]).await
    }
}

impl Drop for TcpSocket<'_, '_> {
    fn drop(&mut self) {
        let slot = self.slot();
        if slot.state.load(Ordering::Acquire) != CLOSED {
            // Cannot wait in drop; if the queue is full the gateway keeps the connection until the
            // board disconnects or the id is reused.
            let _ = self.proxy.try_send(&[OP_CLOSE, self.id]);
        }
        slot.state.store(FREE, Ordering::Release);
    }
}

impl embedded_io_async::ErrorType for TcpSocket<'_, '_> {
    type Error = Error;
}

impl embedded_io_async::Read for TcpSocket<'_, '_> {
    /// Returns 0 once the peer has closed and everything it sent has been read.
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let slot = self.slot();
        loop {
            if !slot.rx.is_empty() {
                let n = slot.rx.read(buf).await;
                self.return_credit(n).await?;
                return Ok(n);
            }
            if slot.state.load(Ordering::Acquire) == CLOSED {
                return Ok(0);
            }
            if let Either::First(n) = select(slot.rx.read(buf), slot.closed.wait()).await {
                self.return_credit(n).await?;
                return Ok(n);
            }
        }
    }
}

impl embedded_io_async::Write for TcpSocket<'_, '_> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        if self.slot().state.load(Ordering::Acquire) != OPEN {
            return Err(Error::Disconnected);
        }
        let n = buf.len().min(self.proxy.max_payload());
        let mut frame: Vec<u8, FRAME_MAX> = Vec::new();
        let _ = frame.extend_from_slice(&[OP_SEND, self.id]);
        let _ = frame.extend_from_slice(&buf[..n]);
        self.proxy.send(&frame).await?;
        Ok(n)
    }

    /// Every write is already queued with the SoftDevice when it returns.
    async fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
