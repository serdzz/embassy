//! MSP430FR504x and FR604x: the ultrasonic sensing family. 64 kB FRAM, 4 kB SRAM.
//!
//! One file for both devices, because they are register-identical. Comparing TI's own headers, the
//! only differences between the MSP430FR6043 and the MSP430FR5043 are the segment LCD driver — the
//! `LCD_C` block, its interrupt vector and its charge-pump bit — and the device name. Every base
//! address the HAL touches is the same, so everything here serves both and the peripheral list in
//! `lib.rs` carries the one difference.
//!
//! The packages differ, 80 pins against 64, so fewer of the pins below are bonded out on the
//! FR5043. That is not something this file can express or needs to: a pin that is not bonded out is
//! still safe to configure, and which ones exist is on the datasheet for the package in hand.
//!
//! # Only part of the FRAM is reachable
//!
//! These are MSP430X devices and their FRAM runs past 0xFFFF: 0x6000 to 0xFF7F is reachable with
//! 16-bit addressing — about 40 kB — and the 24 kB above 0x10000 is not, because Rust's
//! `msp430-none-elf` target has no 20-bit addressing. The linker script must stop at the vector
//! table.
//!
//! # What is not here
//!
//! Nothing about the ultrasonic front end, which is [`crate::uss`]'s business. This file is the
//! chip's own shape: ports, timers and the watchdog.

use super::{PinFunction, PortReg};

/// P1 through P7 can raise interrupts; P8 and P9 cannot.
pub(crate) const IRQ_PORTS: u8 = 7;
/// Every pin has a pull resistor, whose direction the output latch picks.
pub(crate) const HAS_PULL: bool = true;
/// Each interrupt-capable port has a vector register that reports and clears one flag per read.
pub(crate) const HAS_PORT_IV: bool = true;

/// The analog function, which also disconnects the digital input buffer.
pub(crate) const ANALOG_FUNCTION: PinFunction = PinFunction::Alternate3;

/// Timer_B0, the `embassy-time` driver's timer.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_BASE: u16 = 0x03c0;
/// `TB0IV` sits inside the timer's own register block.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_IV: u16 = TB0_BASE + 0x2e;

/// `WDTCTL`, in the WDT_A block.
pub(crate) const WDTCTL: u16 = 0x015c;

/// Base of the port pair `port` belongs to.
///
/// Identical in shape to the FR2xx layout: the ports come in 16-bit pairs — P1 and P2 are the low
/// and high halves of PA, and so on — so the odd-numbered port of a pair uses the pair's addresses
/// directly and the even-numbered one adds a byte.
const fn pair_base(port: u8) -> u16 {
    // PA = P1/P2 at 0x0200, PB = P3/P4 at 0x0220, PC = P5/P6 at 0x0240, PD = P7/P8 at 0x0260,
    // PE = P9 at 0x0280.
    //
    // Shifts rather than a multiply on purpose: MSP430 has no multiply instruction, so `*` would
    // become a call to `__mspabi_mpyi` even here.
    0x0200 + ((port as u16 >> 1) << 5)
}

/// Address of one of `port`'s registers.
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

/// Address of `port`'s interrupt vector register.
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
