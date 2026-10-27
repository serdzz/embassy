#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals, uarte};
use panic_halt as _;

bind_interrupts!(struct Irqs {
    UARTE0 => uarte::InterruptHandler<peripherals::UARTE0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;

    // P0.08 (RXD) and P0.06 (TXD) go to the on-board CP2104, the same port the bootloader uses.
    let mut uart = uarte::Uarte::new(p.UARTE0, p.P0_08, p.P0_06, Irqs, config);

    // Message must be in SRAM
    let mut buf = [0; 23];
    buf.copy_from_slice(b"Hello from embassy-nrf\n");
    uart.write(&buf).await.unwrap();

    // Echo every byte back.
    let mut byte = [0; 1];
    loop {
        uart.read(&mut byte).await.unwrap();
        uart.write(&byte).await.unwrap();
    }
}
