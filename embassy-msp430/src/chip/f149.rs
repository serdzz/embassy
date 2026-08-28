//! MSP430F149: 60 kB flash, 2 kB SRAM.

use super::{PinFunction, PortReg};

/// P1 and P2 can raise interrupts; P3 through P6 cannot.
pub(crate) const IRQ_PORTS: u8 = 2;
/// F1xx has no pull resistors on its port pins. They arrived with the F2xx family.
pub(crate) const HAS_PULL: bool = false;

/// Timer_B7, the `embassy-time` driver's timer.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_BASE: u16 = 0x0180;
/// `TBIV` lives on its own down in the 16-bit peripheral area, nowhere near the timer block.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_IV: u16 = 0x011e;

/// F1xx has no per-port vector register; the handler reads `PxIFG` and clears it itself.
pub(crate) const HAS_PORT_IV: bool = false;

/// F1xx has one select bit, so the converter shares it with everything else on the pin.
pub(crate) const ANALOG_FUNCTION: PinFunction = PinFunction::Alternate1;

/// `WDTCTL`.
pub(crate) const WDTCTL: u16 = 0x0120;

/// Address of one of `port`'s registers, or `None` if this family has no such register.
///
/// The ports are laid out one byte after another: P1 at 0x20, P2 at 0x28, then P3 and P4 down at
/// 0x18, and P5 and P6 at 0x30. The interrupt registers only exist for P1 and P2, which is why the
/// two groups have different strides.
pub(crate) const fn port_reg(port: u8, reg: PortReg) -> Option<*mut u8> {
    let (base, has_irq) = match port {
        0 => (0x0020, true),  // P1
        1 => (0x0028, true),  // P2
        2 => (0x0018, false), // P3
        3 => (0x001c, false), // P4
        4 => (0x0030, false), // P5
        _ => (0x0034, false), // P6
    };

    let offset = match reg {
        PortReg::In => 0,
        PortReg::Out => 1,
        PortReg::Dir => 2,
        // No pull resistors, and the ports without interrupts stop after PxSEL.
        PortReg::Ren => return None,
        PortReg::Ifg => 3,
        PortReg::Ies => 4,
        PortReg::Ie => 5,
    };

    match reg {
        PortReg::Ies | PortReg::Ie | PortReg::Ifg if !has_irq => None,
        _ => Some((base + offset) as *mut u8),
    }
}

/// F1xx has no per-port interrupt vector register: the handler works out which pin fired from
/// `PxIFG` itself.
pub(crate) const fn port_iv(_port: u8) -> *mut u16 {
    core::ptr::null_mut()
}

/// Point a pin at a peripheral, or take it back.
///
/// There is one select bit per pin here, so the only alternate function is the first one.
pub(crate) fn set_pin_function(port: u8, bit: u8, function: PinFunction) {
    let base = match port {
        0 => 0x0020u16,
        1 => 0x0028,
        2 => 0x0018,
        3 => 0x001c,
        4 => 0x0030,
        _ => 0x0034,
    };
    // PxSEL follows the interrupt registers on P1 and P2, and PxDIR on the rest.
    let sel = base + if port < 2 { 6 } else { 3 };

    let on = match function {
        PinFunction::Gpio => false,
        PinFunction::Alternate1 => true,
        other => panic!("{:?} does not exist on this device", other),
    };
    crate::gpio::modify_reg(sel as *mut u8, bit, on);
}
