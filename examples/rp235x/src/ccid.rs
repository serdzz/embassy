//! Minimal USB CCID (smart-card reader) device class for embassy-usb.
//!
//! embassy-usb only ships the *host* side of CCID (`embassy_usb::class::ccid::host`,
//! for a PC reading a CCID device) — there is no device-side implementation to
//! build on, unlike e.g. HID or CDC-ACM. This module implements just enough of
//! CCID rev 1.1 (single slot, short APDU level exchange, bulk-only — no smart
//! card clock/voltage class-specific commands) to present a PN532-backed
//! contactless reader to PC/SC on the host.
//!
//! Reference: USB CCID 1.1 spec (DWG Smart-Card_CCID_Rev110.pdf). Only the
//! subset actually needed for single-slot bulk APDU exchange is implemented.

#![allow(dead_code)]

use embassy_usb::Builder;
use embassy_usb::driver::{Driver, Endpoint, EndpointError, EndpointIn, EndpointOut};
use embassy_usb::types::InterfaceNumber;

/// bInterfaceClass for CCID (Chip/SmartCard Interface Devices)
const CLASS_CCID: u8 = 0x0B;
const SUBCLASS_NONE: u8 = 0x00;
const PROTOCOL_BULK: u8 = 0x00;

/// CCID functional descriptor type (bDescriptorType)
const CCID_FUNCTIONAL_DESCRIPTOR: u8 = 0x21;

// --- PC_to_RDR message types (host -> device, on the bulk-out endpoint) ---
const PC_TO_RDR_ICC_POWER_ON: u8 = 0x62;
const PC_TO_RDR_ICC_POWER_OFF: u8 = 0x63;
const PC_TO_RDR_GET_SLOT_STATUS: u8 = 0x65;
const PC_TO_RDR_XFR_BLOCK: u8 = 0x6F;
const PC_TO_RDR_GET_PARAMETERS: u8 = 0x6C;
const PC_TO_RDR_RESET_PARAMETERS: u8 = 0x6D;
const PC_TO_RDR_SET_PARAMETERS: u8 = 0x61;
const PC_TO_RDR_ESCAPE: u8 = 0x6B;
const PC_TO_RDR_ICC_CLOCK: u8 = 0x6E;
const PC_TO_RDR_ABORT: u8 = 0x72;

// --- RDR_to_PC message types (device -> host, on the bulk-in endpoint) ---
const RDR_TO_PC_DATA_BLOCK: u8 = 0x80;
const RDR_TO_PC_SLOT_STATUS: u8 = 0x81;
const RDR_TO_PC_PARAMETERS: u8 = 0x82;
const RDR_TO_PC_ESCAPE: u8 = 0x83;
const RDR_TO_PC_NOTIFY_SLOT_CHANGE: u8 = 0x50;

/// bStatus (upper 2 bits of bStatus byte in RDR_to_PC messages)
mod slot_status {
    pub const ICC_PRESENT_ACTIVE: u8 = 0x00; // card present, powered
    pub const ICC_PRESENT_INACTIVE: u8 = 0x01; // card present, not powered
    pub const ICC_ABSENT: u8 = 0x02; // no card
}

/// bError + command-failed bit (0x40) in bStatus when something goes wrong
mod slot_error {
    pub const CMD_FAILED: u8 = 0x40;
    pub const ICC_MUTE: u8 = 0xFE; // card present but didn't answer
}

const HEADER_LEN: usize = 10;
/// Max APDU we round-trip: 5-byte header + 255 data + 1 Le (short APDU), or a
/// 256+2 response — keep one buffer big enough for either direction.
pub const MAX_APDU: usize = 261;
const MAX_MSG: usize = HEADER_LEN + MAX_APDU;

/// T=0/T=1 ATR + protocol parameters, as reported in GetParameters/DataBlock.
/// We only support T=0/T=1 pass-through of whatever the PN532 target returns —
/// no real ISO 7816 clocking since this is a contactless (not contact) reader,
/// so `abProtocolDataStructure` fields are mostly placeholders.
#[derive(Clone, Copy)]
pub struct Parameters {
    pub protocol_num: u8, // 0 = T=0, 1 = T=1
    pub data: [u8; 7],    // abProtocolDataStructure, T=1 is 7 bytes; T=0 uses first 5
    pub len: usize,
}

impl Default for Parameters {
    fn default() -> Self {
        // T=1 defaults: Fi/Di=0x11 (372/1, default), TCCKS=0, GuardTime=0,
        // BWI/CWI=0x4A, ClockStop=0, IFSC=0xFE, NADValue=0 — reasonable
        // defaults for a contactless target wrapped in T=1-like framing.
        Self {
            protocol_num: 1,
            data: [0x11, 0x00, 0x00, 0x4A, 0x00, 0xFE, 0x00],
            len: 7,
        }
    }
}

/// What the application layer needs to answer a CCID request.
pub enum HostRequest<'a> {
    /// Host wants to power on slot 0 and get the ATR.
    PowerOn,
    /// Host wants to power off slot 0.
    PowerOff,
    /// Host sent an APDU command to be forwarded to the card; reply with the
    /// card's response bytes (including SW1 SW2).
    Apdu(&'a [u8]),
}

/// Outcome the application layer reports back after a [`HostRequest`].
pub enum HostResponse<'a> {
    /// Card ATR bytes (PowerOn succeeded).
    Atr(&'a [u8]),
    /// Card failed to power on / isn't there (ICC_MUTE).
    PowerOnFailed,
    /// PowerOff acknowledged.
    PoweredOff,
    /// APDU response bytes.
    ApduResponse(&'a [u8]),
    /// APDU exchange failed (treated as a slot-level error, not fatal).
    ApduFailed,
}

/// Slot presence as tracked by the application (poll loop) and pushed into
/// the CCID class so it can send RDR_to_PC_NotifySlotChange.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardPresence {
    Absent,
    PresentInactive,
    PresentActive,
}

struct Control {}

/// Shared state between the CCID class' endpoint handling and the
/// application task that owns the PN532 and answers [`HostRequest`]s.
pub struct State<'d> {
    control: Control,
    _p: core::marker::PhantomData<&'d ()>,
}

impl<'d> Default for State<'d> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'d> State<'d> {
    pub const fn new() -> Self {
        Self {
            control: Control {},
            _p: core::marker::PhantomData,
        }
    }
}

/// The CCID class. Owns the bulk endpoints; the application task calls
/// [`Ccid::recv_request`] / [`Ccid::send_response`] / [`Ccid::notify_card`]
/// in its own loop (there is no Handler-based async dispatch here — CCID's
/// bulk-only transport is simplest as a plain request/response loop owned by
/// the application, see `main.rs`).
pub struct Ccid<'d, D: Driver<'d>> {
    read_ep: D::EndpointOut,
    write_ep: D::EndpointIn,
    notify_ep: D::EndpointIn,
    seq: u8,
    presence: CardPresence,
    params: Parameters,
    _state: &'d State<'d>,
}

impl<'d, D: Driver<'d>> Ccid<'d, D> {
    /// Register the CCID interface (1 bulk-out, 1 bulk-in, 1 interrupt-in
    /// endpoint) and its functional descriptor in the USB config.
    pub fn new(builder: &mut Builder<'d, D>, state: &'d mut State<'d>, max_packet_size: u16) -> Self {
        let mut func = builder.function(CLASS_CCID, SUBCLASS_NONE, PROTOCOL_BULK);
        let mut iface = func.interface();
        let mut alt = iface.alt_setting(CLASS_CCID, SUBCLASS_NONE, PROTOCOL_BULK, None);

        // CCID class descriptor (Smart Card Device Class Descriptor, CCID
        // spec table 5.1). Single slot, short APDU, auto-voltage/-param, no
        // PIN support, bulk transport only.
        alt.descriptor(
            CCID_FUNCTIONAL_DESCRIPTOR,
            &[
                0x10, 0x01, // bcdCCID 1.10
                0x00, // bMaxSlotIndex (1 slot: index 0)
                0x07, // bVoltageSupport: 5V/3V/1.8V
                0x02, 0x00, 0x00, 0x00, // dwProtocols: bit1=T=1 (we present T=1-ish framing)
                0xA0, 0x0F, 0x00, 0x00, // dwDefaultClock (4000 kHz, placeholder)
                0xA0, 0x0F, 0x00, 0x00, // dwMaximumClock
                0x00, // bNumClockSupported (0 = only default)
                0x00, 0x00, 0x00, 0x00, // dwDataRate (placeholder, contactless)
                0x00, 0x00, 0x00, 0x00, // dwMaxDataRate
                0x00, // bNumDataRatesSupported
                0x00, 0x00, 0x00, 0x00, // dwMaxIFSD
                0x00, 0x00, 0x00, 0x00, // dwSynchProtocols
                0x00, 0x00, 0x00, 0x00, // dwMechanical
                // dwFeatures: bit1=auto param config, bit4=auto PPS,
                // bit6=auto IFSD, bit16=short APDU only
                0x10, 0x00, 0x01, 0x00, 0x0F, 0x01, 0x00,
                0x00, // dwMaxCCIDMessageLength (261 + header, placeholder)
                0xFF, // bClassGetResponse
                0xFF, // bClassEnvelope
                0x00, 0x00, // wLcdLayout (none)
                0x00, // bPINSupport (none)
                0x01, // bMaxCCIDBusySlots
            ],
        );

        let read_ep = alt.endpoint_bulk_out(None, max_packet_size);
        let write_ep = alt.endpoint_bulk_in(None, max_packet_size);
        let notify_ep = alt.endpoint_interrupt_in(None, 8, 255);

        Self {
            read_ep,
            write_ep,
            notify_ep,
            seq: 0,
            presence: CardPresence::Absent,
            params: Parameters::default(),
            _state: state,
        }
    }

    /// Wait for the host to enumerate us and open the endpoints.
    pub async fn wait_connection(&mut self) {
        self.read_ep.wait_enabled().await;
    }

    /// Receive one PC_to_RDR message, decode it, and return the application
    /// request it implies (if any). Short commands (GetSlotStatus,
    /// GetParameters, …) are answered here directly and `None` is returned —
    /// the caller should loop and call this again.
    async fn recv_one(&mut self, buf: &mut [u8; MAX_MSG]) -> Result<Option<(u8, u8)>, EndpointError> {
        // CCID bulk-out messages are at most 64-byte packets but the message
        // itself can be longer (dwLength up to MAX_APDU); embassy's bulk
        // endpoint read gives us one packet at a time up to max_packet_size,
        // so read the header first, then the rest if dwLength says more is
        // coming. For simplicity (short APDU only) we assume max_packet_size
        // >= HEADER_LEN and read in a loop until we have the full message.
        let mut got = 0usize;
        loop {
            let end = got + 64.min(buf.len() - got);
            let n = self.read_ep.read(&mut buf[got..end]).await?;
            got += n;
            if got >= HEADER_LEN {
                let dw_length = u32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]) as usize;
                let total = HEADER_LEN + dw_length;
                if got >= total || n < 64 {
                    break;
                }
            } else if n < 64 {
                break;
            }
        }
        if got < HEADER_LEN {
            return Ok(None);
        }
        let msg_type = buf[0];
        let dw_length = u32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]) as usize;
        let slot = buf[5];
        self.seq = buf[6];
        Ok(Some((msg_type, slot)))
    }

    /// Build and send an RDR_to_PC_SlotStatus reply.
    async fn reply_slot_status(&mut self) -> Result<(), EndpointError> {
        let (bstatus, berror) = match self.presence {
            CardPresence::Absent => (slot_status::ICC_ABSENT, 0),
            CardPresence::PresentInactive => (slot_status::ICC_PRESENT_INACTIVE, 0),
            CardPresence::PresentActive => (slot_status::ICC_PRESENT_ACTIVE, 0),
        };
        let mut msg = [0u8; HEADER_LEN];
        msg[0] = RDR_TO_PC_SLOT_STATUS;
        // dwLength = 0
        msg[5] = 0; // bSlot
        msg[6] = self.seq;
        msg[7] = bstatus;
        msg[8] = berror;
        msg[9] = 0; // bClockStatus
        self.write_ep.write(&msg).await
    }

    /// Build and send an RDR_to_PC_DataBlock (used for PowerOn's ATR and for
    /// XfrBlock's APDU response alike).
    async fn reply_data_block(&mut self, data: &[u8], failed: bool) -> Result<(), EndpointError> {
        let mut msg = [0u8; MAX_MSG];
        msg[0] = RDR_TO_PC_DATA_BLOCK;
        let len = data.len() as u32;
        msg[1..5].copy_from_slice(&len.to_le_bytes());
        msg[5] = 0;
        msg[6] = self.seq;
        msg[7] = if failed {
            slot_status::ICC_PRESENT_INACTIVE | slot_error::CMD_FAILED
        } else {
            slot_status::ICC_PRESENT_ACTIVE
        };
        msg[8] = if failed { slot_error::ICC_MUTE } else { 0 };
        msg[9] = 0;
        msg[HEADER_LEN..HEADER_LEN + data.len()].copy_from_slice(data);
        self.write_ep.write(&msg[..HEADER_LEN + data.len()]).await
    }

    async fn reply_parameters(&mut self) -> Result<(), EndpointError> {
        let mut msg = [0u8; HEADER_LEN + 7];
        msg[0] = RDR_TO_PC_PARAMETERS;
        let len = self.params.len as u32;
        msg[1..5].copy_from_slice(&len.to_le_bytes());
        msg[5] = 0;
        msg[6] = self.seq;
        msg[7] = slot_status::ICC_PRESENT_ACTIVE;
        msg[8] = 0;
        msg[9] = self.params.protocol_num;
        msg[HEADER_LEN..HEADER_LEN + self.params.len].copy_from_slice(&self.params.data[..self.params.len]);
        self.write_ep.write(&msg[..HEADER_LEN + self.params.len]).await
    }

    /// Notify the host that card presence changed on slot 0 (RDR_to_PC_NotifySlotChange).
    pub async fn notify_card(&mut self, present: bool) -> Result<(), EndpointError> {
        self.presence = if present {
            CardPresence::PresentInactive
        } else {
            CardPresence::Absent
        };
        // bmSlotICCState: bit0=present, bit1=changed (1 slot => 1 byte)
        let msg = [RDR_TO_PC_NOTIFY_SLOT_CHANGE, if present { 0b11 } else { 0b10 }];
        self.notify_ep.write(&msg).await
    }

    /// One iteration of the device-side protocol: read a PC_to_RDR message,
    /// answer trivial ones (GetSlotStatus/GetParameters/ResetParameters/
    /// SetParameters/IccClock/Escape/Abort) in place, and surface PowerOn /
    /// PowerOff / XfrBlock to the caller as a [`HostRequest`] for the PN532
    /// task to actually service.
    ///
    /// The caller passes the matching [`HostResponse`] into
    /// [`Ccid::finish_request`] once it has one.
    pub async fn run_once<'b>(&mut self, buf: &'b mut [u8; MAX_MSG]) -> Result<Option<HostRequest<'b>>, EndpointError> {
        let Some((msg_type, _slot)) = self.recv_one(buf).await? else {
            return Ok(None);
        };
        match msg_type {
            PC_TO_RDR_GET_SLOT_STATUS => {
                self.reply_slot_status().await?;
                Ok(None)
            }
            PC_TO_RDR_GET_PARAMETERS | PC_TO_RDR_RESET_PARAMETERS => {
                self.reply_parameters().await?;
                Ok(None)
            }
            PC_TO_RDR_SET_PARAMETERS => {
                let dw_length = u32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]) as usize;
                self.params.protocol_num = buf[6 + 1]; // bProtocolNum follows bSeq in this message
                self.params.len = dw_length.min(self.params.data.len());
                self.params.data[..self.params.len].copy_from_slice(&buf[HEADER_LEN..HEADER_LEN + self.params.len]);
                self.reply_parameters().await?;
                Ok(None)
            }
            PC_TO_RDR_ICC_CLOCK | PC_TO_RDR_ABORT => {
                self.reply_slot_status().await?;
                Ok(None)
            }
            PC_TO_RDR_ESCAPE => {
                // No vendor escape commands implemented; echo back empty + CMD_FAILED
                self.reply_data_block(&[], true).await?;
                Ok(None)
            }
            PC_TO_RDR_ICC_POWER_ON => Ok(Some(HostRequest::PowerOn)),
            PC_TO_RDR_ICC_POWER_OFF => Ok(Some(HostRequest::PowerOff)),
            PC_TO_RDR_XFR_BLOCK => {
                let dw_length = u32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]) as usize;
                Ok(Some(HostRequest::Apdu(&buf[HEADER_LEN..HEADER_LEN + dw_length])))
            }
            _ => {
                // Unsupported command: answer with a generic slot-status error.
                self.reply_slot_status().await?;
                Ok(None)
            }
        }
    }

    /// Send the reply matching a [`HostRequest`] previously returned by
    /// [`Ccid::run_once`].
    pub async fn finish_request(&mut self, response: HostResponse<'_>) -> Result<(), EndpointError> {
        match response {
            HostResponse::Atr(atr) => {
                self.presence = CardPresence::PresentActive;
                self.reply_data_block(atr, false).await
            }
            HostResponse::PowerOnFailed => {
                self.presence = CardPresence::PresentInactive;
                self.reply_data_block(&[], true).await
            }
            HostResponse::PoweredOff => {
                self.presence = CardPresence::PresentInactive;
                self.reply_slot_status().await
            }
            HostResponse::ApduResponse(data) => self.reply_data_block(data, false).await,
            HostResponse::ApduFailed => self.reply_data_block(&[], true).await,
        }
    }
}
