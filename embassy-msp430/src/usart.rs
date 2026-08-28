//! Plumbing shared by both USART modes.
//!
//! A USART can be a UART or an SPI, but only one at a time, and whichever it is it raises the same
//! two interrupt vectors. So the vectors live here rather than in either driver, and do the one
//! thing that is right in both modes: mask whatever asserted and wake the task, leaving the flags
//! themselves alone. The flags belong to the driver — they are cleared by moving the data, which is
//! not the handler's to move.

use embassy_sync::waitqueue::AtomicWaker;

// Special function registers, which is where this family keeps the module enables and the interrupt
// enables and flags rather than in the peripheral itself.
const IE1: u16 = 0x0000;
const IE2: u16 = 0x0001;
const IFG1: u16 = 0x0002;
const IFG2: u16 = 0x0003;
const ME1: u16 = 0x0004;
const ME2: u16 = 0x0005;

/// Two modules, each with two waker slots. UART hands one to its receive half and one to its
/// transmit half, so the two can be awaited from separate tasks; SPI only needs the first.
const WAKER_COUNT: usize = 4;

#[allow(clippy::declare_interior_mutable_const)]
const NEW_AW: AtomicWaker = AtomicWaker::new();
static WAKERS: [AtomicWaker; WAKER_COUNT] = [NEW_AW; WAKER_COUNT];

/// Where one USART lives, and which bits in the special function registers belong to it.
pub(crate) struct Info {
    /// Peripheral base address.
    pub base: u16,
    /// The `MEx` register this instance's enable bits live in.
    pub sfr_enable: u16,
    /// The `IEx` register.
    pub sfr_ie: u16,
    /// The `IFGx` register.
    pub sfr_ifg: u16,
    /// Receive and transmit bits, which occupy the same positions in all three of those.
    pub rx_bit: u8,
    pub tx_bit: u8,
    /// In synchronous mode one bit enables the whole module, rather than one per direction.
    pub spi_enable_bit: u8,
    /// Module index, 0 or 1.
    pub idx: usize,
}

#[inline]
pub(crate) fn read8(addr: u16) -> u8 {
    // SAFETY: a volatile read of a USART or special function register.
    unsafe { (addr as *mut u8).read_volatile() }
}

#[inline]
pub(crate) fn write8(addr: u16, value: u8) {
    // SAFETY: a volatile write to a USART or special function register.
    unsafe { (addr as *mut u8).write_volatile(value) }
}

#[inline]
pub(crate) fn modify8(addr: u16, f: impl FnOnce(u8) -> u8) {
    critical_section::with(|_| write8(addr, f(read8(addr))));
}

impl Info {
    #[inline]
    pub fn read(&self, offset: u16) -> u8 {
        read8(self.base + offset)
    }

    #[inline]
    pub fn write(&self, offset: u16, value: u8) {
        write8(self.base + offset, value);
    }

    #[inline]
    pub fn modify(&self, offset: u16, f: impl FnOnce(u8) -> u8) {
        modify8(self.base + offset, f);
    }

    /// Is one of this instance's interrupt flags up?
    #[inline]
    pub fn flag(&self, bit: u8) -> bool {
        read8(self.sfr_ifg) & bit != 0
    }

    #[inline]
    pub fn enable_irq(&self, bits: u8) {
        modify8(self.sfr_ie, |v| v | bits);
    }

    #[inline]
    pub fn disable_irq(&self, bits: u8) {
        modify8(self.sfr_ie, |v| v & !bits);
    }

    /// Connect or disconnect the module. Without this the pins stay disconnected however the port
    /// is configured.
    #[inline]
    pub fn set_enabled(&self, bits: u8, on: bool) {
        modify8(self.sfr_enable, |v| if on { v | bits } else { v & !bits });
    }

    /// Waker `n` (0 or 1) of this module.
    #[inline]
    pub fn waker(&self, n: usize) -> &'static AtomicWaker {
        &WAKERS[self.idx * 2 + n]
    }
}

pub(crate) static INFO_U0: Info = Info {
    base: 0x0070,
    sfr_enable: ME1,
    sfr_ie: IE1,
    sfr_ifg: IFG1,
    rx_bit: 0x40,
    tx_bit: 0x80,
    spi_enable_bit: 0x40,
    idx: 0,
};

pub(crate) static INFO_U1: Info = Info {
    base: 0x0078,
    sfr_enable: ME2,
    sfr_ie: IE2,
    sfr_ifg: IFG2,
    rx_bit: 0x10,
    tx_bit: 0x20,
    spi_enable_bit: 0x10,
    idx: 1,
};

/// Mask the source and wake the module's tasks.
///
/// Masking rather than clearing is what makes this work for both modes: the flag stays up as the
/// "ready" answer the future polls for, and cannot re-enter the handler in the meantime. Both
/// wakers are woken because the handler has no idea which mode the module is in; a task woken for
/// nothing simply polls again.
fn on_irq(info: &'static Info, bit: u8) {
    if read8(info.sfr_ie) & bit == 0 || !info.flag(bit) {
        return;
    }
    info.disable_irq(bit);
    info.waker(0).wake();
    info.waker(1).wake();
}

embassy_executor::msp430_interrupt! {
    /// USART0 receive, in whichever mode it is configured.
    unsafe fn USART0RX() {
        on_irq(&INFO_U0, INFO_U0.rx_bit);
    }

    /// USART0 transmit, in whichever mode it is configured.
    unsafe fn USART0TX() {
        on_irq(&INFO_U0, INFO_U0.tx_bit);
    }

    /// USART1 receive, in whichever mode it is configured.
    unsafe fn USART1RX() {
        on_irq(&INFO_U1, INFO_U1.rx_bit);
    }

    /// USART1 transmit, in whichever mode it is configured.
    unsafe fn USART1TX() {
        on_irq(&INFO_U1, INFO_U1.tx_bit);
    }
}
