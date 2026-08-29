//! Plumbing shared by every eUSCI mode.
//!
//! One eUSCI module can be a UART, an SPI or an I2C, but only one at a time, and whichever mode it
//! is in it raises the same interrupt vector. So the vector lives here rather than in any one
//! driver, and does the one thing that is right in every mode: mask whatever asserted and wake the
//! task, leaving the flags themselves alone. The flags belong to the driver — `UCRXIFG` is cleared
//! by reading `RXBUF` and `UCTXIFG` by writing `TXBUF`, both of which move data the handler has no
//! business touching.

use embassy_sync::waitqueue::AtomicWaker;

/// Four modules, each with two waker slots. UART hands one to its receive half and one to its
/// transmit half, so the two can be awaited from separate tasks; SPI and I2C only need the first.
/// eUSCI instances on this device: four on the FR2355, six on the FR6043.
#[cfg(feature = "msp430fr2355")]
pub(crate) const INSTANCES: usize = 4;
/// See [`INSTANCES`].
#[cfg(feature = "_fr504x_604x")]
pub(crate) const INSTANCES: usize = 6;

/// Two waker slots per instance: one for the receiving half and one for the transmitting half, so
/// that the two can be awaited from separate tasks.
const WAKER_COUNT: usize = INSTANCES * 2;

#[allow(clippy::declare_interior_mutable_const)]
const NEW_AW: AtomicWaker = AtomicWaker::new();
static WAKERS: [AtomicWaker; WAKER_COUNT] = [NEW_AW; WAKER_COUNT];

/// Where one eUSCI module lives.
pub(crate) struct Info {
    /// Peripheral base address.
    pub base: u16,
    /// Offset of `UCxIE`. eUSCI_A puts it at 0x1A, eUSCI_B at 0x2A.
    pub ie_off: u16,
    /// Offset of `UCxIFG`.
    pub ifg_off: u16,
    /// Module index, 0 to 3.
    pub idx: usize,
}

impl Info {
    #[inline]
    fn reg(&self, offset: u16) -> *mut u16 {
        (self.base + offset) as *mut u16
    }

    #[inline]
    pub fn read(&self, offset: u16) -> u16 {
        // SAFETY: a volatile read of a register of an existing eUSCI module.
        unsafe { self.reg(offset).read_volatile() }
    }

    #[inline]
    pub fn write(&self, offset: u16, value: u16) {
        // SAFETY: a volatile write to a register of an existing eUSCI module.
        unsafe { self.reg(offset).write_volatile(value) }
    }

    #[inline]
    pub fn modify(&self, offset: u16, f: impl FnOnce(u16) -> u16) {
        critical_section::with(|_| {
            let v = self.read(offset);
            self.write(offset, f(v));
        })
    }

    #[inline]
    pub fn ie(&self) -> u16 {
        self.read(self.ie_off)
    }

    #[inline]
    pub fn ifg(&self) -> u16 {
        self.read(self.ifg_off)
    }

    #[inline]
    pub fn enable_irq(&self, bits: u16) {
        self.modify(self.ie_off, |v| v | bits);
    }

    #[inline]
    pub fn disable_irq(&self, bits: u16) {
        self.modify(self.ie_off, |v| v & !bits);
    }

    #[inline]
    pub fn clear_flags(&self, bits: u16) {
        self.modify(self.ifg_off, |v| v & !bits);
    }

    /// Waker `n` (0 or 1) of this module.
    #[inline]
    pub fn waker(&self, n: usize) -> &'static AtomicWaker {
        &WAKERS[self.idx * 2 + n]
    }
}

// The eUSCI module is identical across these two devices, down to the register offsets. Only where
// the instances sit differs, and how many there are.
//
// `idx` is this crate's own numbering, used to pick a waker pair; it has nothing to do with the
// module's name.

#[cfg(feature = "msp430fr2355")]
pub(crate) static INFO_A0: Info = Info {
    base: 0x0500,
    ie_off: 0x1a,
    ifg_off: 0x1c,
    idx: 0,
};
#[cfg(feature = "msp430fr2355")]
pub(crate) static INFO_A1: Info = Info {
    base: 0x0580,
    ie_off: 0x1a,
    ifg_off: 0x1c,
    idx: 1,
};
#[cfg(feature = "msp430fr2355")]
pub(crate) static INFO_B0: Info = Info {
    base: 0x0540,
    ie_off: 0x2a,
    ifg_off: 0x2c,
    idx: 2,
};
#[cfg(feature = "msp430fr2355")]
pub(crate) static INFO_B1: Info = Info {
    base: 0x05c0,
    ie_off: 0x2a,
    ifg_off: 0x2c,
    idx: 3,
};

#[cfg(feature = "_fr504x_604x")]
pub(crate) static INFO_A0: Info = Info {
    base: 0x05c0,
    ie_off: 0x1a,
    ifg_off: 0x1c,
    idx: 0,
};
#[cfg(feature = "_fr504x_604x")]
pub(crate) static INFO_A1: Info = Info {
    base: 0x05e0,
    ie_off: 0x1a,
    ifg_off: 0x1c,
    idx: 1,
};
#[cfg(feature = "_fr504x_604x")]
pub(crate) static INFO_A2: Info = Info {
    base: 0x0600,
    ie_off: 0x1a,
    ifg_off: 0x1c,
    idx: 2,
};
#[cfg(feature = "_fr504x_604x")]
pub(crate) static INFO_A3: Info = Info {
    base: 0x0620,
    ie_off: 0x1a,
    ifg_off: 0x1c,
    idx: 3,
};
#[cfg(feature = "_fr504x_604x")]
pub(crate) static INFO_B0: Info = Info {
    base: 0x0640,
    ie_off: 0x2a,
    ifg_off: 0x2c,
    idx: 4,
};
#[cfg(feature = "_fr504x_604x")]
pub(crate) static INFO_B1: Info = Info {
    base: 0x0680,
    ie_off: 0x2a,
    ifg_off: 0x2c,
    idx: 5,
};

/// Mask every source that is both enabled and asserted, then wake the module's tasks.
///
/// Masking rather than clearing is what makes this work for all three modes: the flag stays up as
/// the "ready" answer the future polls for, and cannot re-enter the handler in the meantime.
fn on_irq(info: &'static Info) {
    let active = info.ie() & info.ifg();
    if active == 0 {
        return;
    }
    info.disable_irq(active);
    info.waker(0).wake();
    info.waker(1).wake();
}

embassy_executor::msp430_interrupt! {
    /// eUSCI_A0, in whichever mode it is configured.
    unsafe fn EUSCI_A0() {
        on_irq(&INFO_A0);
    }

    /// eUSCI_A1, in whichever mode it is configured.
    unsafe fn EUSCI_A1() {
        on_irq(&INFO_A1);
    }

    /// eUSCI_B0, in whichever mode it is configured.
    unsafe fn EUSCI_B0() {
        on_irq(&INFO_B0);
    }

    /// eUSCI_B1, in whichever mode it is configured.
    unsafe fn EUSCI_B1() {
        on_irq(&INFO_B1);
    }
}

#[cfg(feature = "_fr504x_604x")]
embassy_executor::msp430_interrupt! {
    /// eUSCI_A2, in whichever mode it is configured.
    unsafe fn EUSCI_A2() {
        on_irq(&INFO_A2);
    }

    /// eUSCI_A3, in whichever mode it is configured.
    unsafe fn EUSCI_A3() {
        on_irq(&INFO_A3);
    }
}
