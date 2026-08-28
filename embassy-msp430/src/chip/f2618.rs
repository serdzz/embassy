//! MSP430F2618: 116 kB flash, 8 kB SRAM.
//!
//! # Only half the flash is reachable
//!
//! This device is an MSP430X: its flash runs from 0x3100 to 0x1FFFF, and the part above 0xFFFF can
//! only be reached with 20-bit addressing. Rust's `msp430-none-elf` target is 16-bit throughout, so
//! **the upper 64 kB is unusable from Rust** and the linker script stops at 0xFFC0. That leaves
//! about 51 kB, which is the budget to build within.

use super::{PinFunction, PortReg};

/// P1 through P8.
pub(crate) const IRQ_PORTS: u8 = 2;
/// F2xx introduced the pull resistors that F1xx lacks.
pub(crate) const HAS_PULL: bool = true;
/// No per-port vector register on this family either; the handler reads `PxIFG`.
pub(crate) const HAS_PORT_IV: bool = false;

/// The analog function is just "the peripheral": one select bit per pin, as on F1xx.
pub(crate) const ANALOG_FUNCTION: PinFunction = PinFunction::Alternate1;

/// Timer_B7, the `embassy-time` driver's timer.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_BASE: u16 = 0x0180;
/// `TBIV` lives on its own down in the 16-bit peripheral area, as on F1xx.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_IV: u16 = 0x011e;

/// `WDTCTL`.
pub(crate) const WDTCTL: u16 = 0x0120;

/// Address of one of `port`'s registers, or `None` if this family has no such register.
///
/// The layout is the untidiest of the three families. P1 and P2 have all eight registers in one run
/// because they are the ones that can interrupt; the rest have four, split between two areas, with
/// their resistor-enable registers gathered together at 0x10; and P7 and P8 are interleaved into
/// word pairs. So this is a table rather than arithmetic.
pub(crate) const fn port_reg(port: u8, reg: PortReg) -> Option<*mut u8> {
    let addr: u16 = match (port, reg) {
        // P1 and P2: eight consecutive registers each.
        (0, PortReg::In) => 0x20,
        (0, PortReg::Out) => 0x21,
        (0, PortReg::Dir) => 0x22,
        (0, PortReg::Ifg) => 0x23,
        (0, PortReg::Ies) => 0x24,
        (0, PortReg::Ie) => 0x25,
        (0, PortReg::Ren) => 0x27,
        (1, PortReg::In) => 0x28,
        (1, PortReg::Out) => 0x29,
        (1, PortReg::Dir) => 0x2a,
        (1, PortReg::Ifg) => 0x2b,
        (1, PortReg::Ies) => 0x2c,
        (1, PortReg::Ie) => 0x2d,
        (1, PortReg::Ren) => 0x2f,

        // P3 through P6: input, output and direction together, resistor enable off at 0x10.
        (2, PortReg::In) => 0x18,
        (2, PortReg::Out) => 0x19,
        (2, PortReg::Dir) => 0x1a,
        (2, PortReg::Ren) => 0x10,
        (3, PortReg::In) => 0x1c,
        (3, PortReg::Out) => 0x1d,
        (3, PortReg::Dir) => 0x1e,
        (3, PortReg::Ren) => 0x11,
        (4, PortReg::In) => 0x30,
        (4, PortReg::Out) => 0x31,
        (4, PortReg::Dir) => 0x32,
        (4, PortReg::Ren) => 0x12,
        (5, PortReg::In) => 0x34,
        (5, PortReg::Out) => 0x35,
        (5, PortReg::Dir) => 0x36,
        (5, PortReg::Ren) => 0x13,

        // P7 and P8 share word addresses, so their registers alternate.
        (6, PortReg::In) => 0x38,
        (6, PortReg::Out) => 0x3a,
        (6, PortReg::Dir) => 0x3c,
        (6, PortReg::Ren) => 0x14,
        (7, PortReg::In) => 0x39,
        (7, PortReg::Out) => 0x3b,
        (7, PortReg::Dir) => 0x3d,
        (7, PortReg::Ren) => 0x15,

        // Only P1 and P2 can interrupt.
        _ => return None,
    };
    Some(addr as *mut u8)
}

/// F2xx has no per-port interrupt vector register.
pub(crate) const fn port_iv(_port: u8) -> *mut u16 {
    core::ptr::null_mut()
}

/// Address of `port`'s function-select register.
const fn sel_reg(port: u8) -> u16 {
    match port {
        0 => 0x26,
        1 => 0x2e,
        2 => 0x1b,
        3 => 0x1f,
        4 => 0x33,
        5 => 0x37,
        6 => 0x3e,
        _ => 0x3f,
    }
}

/// Point a pin at a peripheral, or take it back.
///
/// One select bit per pin, so the only alternate function is the first one.
pub(crate) fn set_pin_function(port: u8, bit: u8, function: PinFunction) {
    let on = match function {
        PinFunction::Gpio => false,
        PinFunction::Alternate1 => true,
        other => panic!("{:?} does not exist on this device", other),
    };
    crate::gpio::modify_reg(sel_reg(port) as *mut u8, bit, on);
}
