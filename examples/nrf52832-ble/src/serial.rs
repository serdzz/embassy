//! Logging and panics over the USB serial port (UARTE0 on P0.06/P0.08, 115200 baud).
//!
//! The board has no debug probe attached, so there is no defmt; `log!` formats a line into a queue
//! that `serial_task` drains to the UART.

use core::fmt::Write;
use core::panic::PanicInfo;

use embassy_nrf::interrupt::{InterruptExt, Priority};
use embassy_nrf::{Peri, bind_interrupts, interrupt, peripherals, uarte};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use heapless::String;

pub type Line = String<96>;

pub static LOG: Channel<CriticalSectionRawMutex, Line, 16> = Channel::new();

macro_rules! log {
    ($($arg:tt)*) => {{
        let mut line = $crate::serial::Line::new();
        let _ = core::fmt::Write::write_fmt(&mut line, format_args!($($arg)*));
        let _ = core::fmt::Write::write_str(&mut line, "\r\n");
        // Drop the line if the queue is full, rather than block the caller.
        let _ = $crate::serial::LOG.try_send(line);
    }};
}

bind_interrupts!(struct Irqs {
    UARTE0 => uarte::InterruptHandler<peripherals::UARTE0>;
});

#[embassy_executor::task]
pub async fn serial_task(
    uarte: Peri<'static, peripherals::UARTE0>,
    txd: Peri<'static, peripherals::P0_06>,
) -> ! {
    // Priorities 0, 1 and 4 belong to the SoftDevice.
    interrupt::UARTE0.set_priority(Priority::P3);
    let mut config = uarte::Config::default();
    config.baudrate = uarte::Baudrate::BAUD115200;
    let mut uart = uarte::UarteTx::new(uarte, Irqs, txd, config);
    loop {
        let line = LOG.receive().await;
        let _ = uart.write(line.as_bytes()).await;
    }
}

// Panics bypass the driver: interrupts are off by then, so write UART0 registers directly.
const UART0: usize = 0x4000_2000;

fn reg(offset: usize) -> *mut u32 {
    (UART0 + offset) as *mut u32
}

struct RawUart;

impl Write for RawUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            unsafe {
                reg(0x11C).write_volatile(0); // EVENTS_TXDRDY
                reg(0x51C).write_volatile(b as u32); // TXD
                while reg(0x11C).read_volatile() == 0 {}
            }
        }
        Ok(())
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    unsafe {
        reg(0x500).write_volatile(0); // ENABLE: release the pins from UARTE
        reg(0x50C).write_volatile(6); // PSELTXD
        reg(0x524).write_volatile(0x01D7_E000); // BAUDRATE 115200
        reg(0x500).write_volatile(4); // ENABLE: UART
        reg(0x008).write_volatile(1); // TASKS_STARTTX
    }
    let _ = write!(RawUart, "\r\npanic: {}\r\n", info);
    loop {
        cortex_m::asm::wfi();
    }
}
