//! Buttonless DFU over BLE, handing off to the Adafruit bootloader.
//!
//! The application exposes Nordic's legacy (SDK 11) DFU service in "application mode". A DFU client
//! (nRF Connect, nRF Toolbox, Bluefruit Connect, or `dfu_ble.py` next to this crate) writes START_DFU
//! to the control point; the application then sets GPREGRET to the bootloader's "OTA DFU" magic and
//! resets. The bootloader starts the SoftDevice itself, advertises the same DFU service, and the
//! client reconnects and uploads the new image.

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use heapless::Vec;
use nrf_softdevice::raw;

/// GPREGRET value that makes the Adafruit bootloader start in BLE (OTA) DFU mode after a reset.
const DFU_MAGIC_OTA_RESET: u32 = 0xA8;

/// Control point opcode a DFU client writes to switch an application into the bootloader.
const OP_START_DFU: u8 = 0x01;

#[nrf_softdevice::gatt_service(uuid = "00001530-1212-efde-1523-785feabcd123")]
pub struct DfuService {
    /// Image data. Only used in bootloader mode; present so clients recognise the service.
    #[characteristic(uuid = "00001532-1212-efde-1523-785feabcd123", write_without_response)]
    pub packet: Vec<u8, 20>,
    #[characteristic(uuid = "00001531-1212-efde-1523-785feabcd123", write, notify)]
    pub control: Vec<u8, 20>,
    /// DFU revision 0.1 tells clients that this is an application, not the bootloader, so they
    /// send START_DFU here and then reconnect.
    #[characteristic(uuid = "00001534-1212-efde-1523-785feabcd123", read, value = "[0x01, 0x00]")]
    pub revision: u16,
}

/// Set from the GATT event handler when a client asks for DFU; the connection loop waits on it.
pub static REQUESTED: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Call from the GATT server's event handler.
pub fn on_event(event: DfuServiceEvent) {
    if let DfuServiceEvent::ControlWrite(data) = event {
        if data.first() == Some(&OP_START_DFU) {
            REQUESTED.signal(());
        }
    }
}

/// Reset into the bootloader's BLE DFU mode.
pub fn enter_bootloader() -> ! {
    // POWER belongs to the SoftDevice while it is enabled, so GPREGRET goes through it.
    unsafe {
        raw::sd_power_gpregret_clr(0, 0xFF);
        raw::sd_power_gpregret_set(0, DFU_MAGIC_OTA_RESET);
    }
    cortex_m::peripheral::SCB::sys_reset()
}
