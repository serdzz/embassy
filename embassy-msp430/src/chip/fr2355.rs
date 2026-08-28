//! MSP430FR2355: 32 kB FRAM, 4 kB SRAM.

use super::{PinFunction, PortReg};

/// P1 through P4 can raise interrupts; P5 and P6 cannot.
pub(crate) const IRQ_PORTS: u8 = 4;
/// Every pin has a pull resistor, whose direction the output latch picks.
pub(crate) const HAS_PULL: bool = true;

/// Timer_B0, the `embassy-time` driver's timer.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_BASE: u16 = 0x0380;
/// `TB0IV` sits inside the timer's own register block.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_IV: u16 = TB0_BASE + 0x2e;

/// FR2xx gives every interrupt-capable port a vector register that reports and clears one
/// flag in a single read.
pub(crate) const HAS_PORT_IV: bool = true;

/// `WDTCTL`.
pub(crate) const WDTCTL: u16 = 0x01cc;

/// Base of the port pair `port` belongs to.
///
/// The ports come in 16-bit pairs — P1 and P2 are the low and high halves of PA, and so on — so the
/// odd-numbered port of a pair uses the pair's addresses directly and the even-numbered one adds a
/// byte.
const fn pair_base(port: u8) -> u16 {
    // PA = P1/P2 at 0x0200, PB = P3/P4 at 0x0220, PC = P5/P6 at 0x0240.
    //
    // Shifts rather than a multiply on purpose: MSP430 has no multiply instruction, so `*` would
    // become a call to `__mspabi_mpyi` even here.
    0x0200 + ((port as u16 >> 1) << 5)
}

/// Address of one of `port`'s registers, or `None` if this family has no such register.
pub(crate) const fn port_reg(port: u8, reg: PortReg) -> Option<*mut u8> {
    let offset = match reg {
        PortReg::In => 0x00,
        PortReg::Out => 0x02,
        PortReg::Dir => 0x04,
        PortReg::Ren => 0x06,
        PortReg::Ies => 0x18,
        PortReg::Ie => 0x1a,
        PortReg::Ifg => 0x1c,
    };
    Some((pair_base(port) + offset + (port as u16 & 1)) as *mut u8)
}

/// Address of `port`'s interrupt vector register, which reports and clears the highest-priority
/// pending flag in one read.
pub(crate) const fn port_iv(port: u8) -> *mut u16 {
    (pair_base(port) + if port & 1 == 0 { 0x0e } else { 0x1e }) as *mut u16
}

/// Point a pin at a peripheral, or take it back.
pub(crate) fn set_pin_function(port: u8, bit: u8, function: PinFunction) {
    const SEL0: u16 = 0x0a;
    const SEL1: u16 = 0x0c;

    let (sel0, sel1) = match function {
        PinFunction::Gpio => (false, false),
        PinFunction::Alternate1 => (true, false),
        PinFunction::Alternate2 => (false, true),
        PinFunction::Alternate3 => (true, true),
    };

    let base = pair_base(port) + (port as u16 & 1);
    crate::gpio::modify_reg((base + SEL0) as *mut u8, bit, sel0);
    crate::gpio::modify_reg((base + SEL1) as *mut u8, bit, sel1);
}
