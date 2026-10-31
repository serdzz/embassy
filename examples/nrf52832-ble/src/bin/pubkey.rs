//! Compile-time-baked OpenHaystack public key, and the two derived
//! values Apple Find My needs from it (BLE device address, advertisement
//! payload) — see the module doc comment in `airtag.rs` for the overall
//! flow and where this key comes from (OpenHaystack's `generate_keys.py`
//! on a Mac, NOT generated here).
//!
//! # Getting `AIRTAG_PUBKEY_HEX`
//! On a Mac, after cloning <https://github.com/seemoo-lab/openhaystack>
//! and running its key generator, you'll have a Base64 string (the
//! "advertisement key"). Convert it to the hex this firmware expects:
//! ```sh
//! python3 -c "import base64,sys; print(base64.b64decode(sys.argv[1]).hex())" "<base64-key>"
//! ```
//! That prints 56 hex characters (28 bytes) — pass them as:
//! ```sh
//! AIRTAG_PUBKEY_HEX=<56 hex chars> cargo build --release --bin airtag
//! ```

/// The 28-byte OpenHaystack public key, decoded at compile time from the
/// `AIRTAG_PUBKEY_HEX` env var. `option_env!` + a const-context hex
/// decoder (same pattern as `lora-dx-lr30`'s `device_id.rs`) rather than
/// a build.rs step — keeps this a single self-contained file.
pub const PUBLIC_KEY: [u8; 28] = {
    match option_env!("AIRTAG_PUBKEY_HEX") {
        Some(s) => decode_hex_28_or_panic(s),
        None => {
            // No key baked in — advertise all-zero. Find My will not
            // recognise this as a valid Offline Finding report (it's not
            // a real key), so the device just advertises harmlessly
            // instead of failing to build. Set AIRTAG_PUBKEY_HEX for a
            // real deployment.
            [0u8; 28]
        }
    }
};

const fn decode_hex_28_or_panic(s: &str) -> [u8; 28] {
    let bytes = s.as_bytes();
    if bytes.len() != 56 {
        panic!("AIRTAG_PUBKEY_HEX must be exactly 56 hex characters (28 bytes)");
    }
    let mut out = [0u8; 28];
    let mut i = 0;
    while i < 28 {
        let hi = hex_nibble(bytes[i * 2]);
        let lo = hex_nibble(bytes[i * 2 + 1]);
        out[i] = (hi << 4) | lo;
        i += 1;
    }
    out
}

const fn hex_nibble(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => panic!("AIRTAG_PUBKEY_HEX must be hex digits only (0-9, a-f)"),
    }
}

/// Derives this device's BLE device address from the public key: top 2
/// bits forced to `11` (making it a valid "random static" address per
/// the Bluetooth spec — SoftDevice would otherwise reject an address
/// whose top bits don't match its declared `AddressType`), remaining 46
/// bits taken directly from `PUBLIC_KEY[0..6]`. Exact algorithm from
/// OpenHaystack's ESP32 reference firmware (`set_addr_from_key`) — Find
/// My correlates the advertisement payload with the sender's address, so
/// deviating from this derivation means a receiving iPhone won't
/// associate the two correctly.
///
/// Returned least-significant-byte-first (`[0]` = key byte 0, matching
/// `nrf_softdevice::ble::Address::bytes`'s own LSB-first convention) —
/// double-check against a packet capture if BLE addresses ever look
/// reversed on the air; this hasn't been checked against a real capture,
/// only against the reference C source's byte order.
pub fn ble_address() -> [u8; 6] {
    // nrf-softdevice's `Address` bytes are little-endian (`[0]` = LSB, `[5]` = MSB),
    // and a RandomStatic address must have its two most-significant bits (top of the
    // MSB, i.e. `[5]`) set to `11`. OpenHaystack's address *value* is `PUBLIC_KEY[0..6]`
    // big-endian with `PUBLIC_KEY[0]`'s top two bits forced to `11`. So reverse the six
    // key bytes into the little-endian array and set the top bits on the MSB (`addr[5]`).
    let mut addr = [0u8; 6];
    addr[0] = PUBLIC_KEY[5];
    addr[1] = PUBLIC_KEY[4];
    addr[2] = PUBLIC_KEY[3];
    addr[3] = PUBLIC_KEY[2];
    addr[4] = PUBLIC_KEY[1];
    addr[5] = PUBLIC_KEY[0] | 0b1100_0000;
    addr
}

/// Builds the full BLE advertisement payload: the manufacturer-specific
/// "Offline Finding" AD structure, byte-for-byte matching OpenHaystack's
/// ESP32 reference firmware's `adv_data[31]` (see that project's
/// `openhaystack_main.c`) — NO separate Flags AD structure (the
/// reference firmware's `adv_data` array is exactly these 31 bytes and
/// nothing else, which is non-standard but is what Find My actually
/// recognises; adding a Flags structure in front would also blow the
/// 31-byte legacy advertising cap once combined with this payload).
///
/// Layout: `[0]` length (0x1E=30, counts everything after itself),
/// `[1]` AD type 0xFF (Manufacturer Specific Data), `[2..4]` Apple
/// company ID 0x004C (little-endian), `[4]` Offline Finding type 0x12,
/// `[5]` Offline Finding length 0x19 (a reverse-engineered protocol
/// constant, not a byte count that needs to add up on its own),
/// `[6]` status byte (0 = normal), `[7..29]` the key's last 22 bytes,
/// `[29]` the key's top 2 bits, `[30]` reserved/always-zero padding —
/// present in the reference array but never written, kept here for
/// exact byte-for-byte parity with the known-working reference payload.
pub fn advertisement_payload() -> [u8; 31] {
    let mut buf = [0u8; 31];
    buf[0] = 0x1E;
    buf[1] = 0xFF;
    buf[2] = 0x4C;
    buf[3] = 0x00;
    buf[4] = 0x12;
    buf[5] = 0x19;
    buf[6] = 0x00;
    buf[7..29].copy_from_slice(&PUBLIC_KEY[6..28]);
    buf[29] = PUBLIC_KEY[0] >> 6;
    // buf[30] stays 0x00 — see doc comment.
    buf
}
