//! MSP430FR4133: 15 kB FRAM, 2 kB SRAM, and the segment LCD driver — the MSP-EXP430FR4133
//! LaunchPad's chip.
//!
//! The ports are laid out exactly as on the FR2xx — 16-bit pairs high in the address map, pull
//! resistors everywhere, vector registers on the interrupt-capable ports — with one difference
//! that matters here: each pin has a *single* function-select bit (`PxSEL0`; there is no `PxSEL1`
//! on this family), so only [`PinFunction::Gpio`] and [`PinFunction::Alternate1`] exist as pin
//! states. The converter's inputs are not a select encoding at all: they are routed by the
//! `ADCPCTLx` bits in `SYSCFG2`, which is what [`ANALOG_FUNCTION`] turns into below.

use super::{PinFunction, PortReg};

/// P1 and P2 can raise interrupts; P3 through P8 cannot.
pub(crate) const IRQ_PORTS: u8 = 2;
/// Every pin has a pull resistor, whose direction the output latch picks.
pub(crate) const HAS_PULL: bool = true;

/// Timer0_A3, the `embassy-time` driver's timer. The FR4xx has no Timer_B at all, so the shared
/// driver — written against Timer_B register names, which Timer_A matches bit for bit in every
/// field the driver touches — runs on TA0 instead.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_BASE: u16 = 0x0300;
/// `TA0IV` sits inside the timer's own register block.
#[cfg(feature = "_time-driver")]
pub(crate) const TB0_IV: u16 = TB0_BASE + 0x2e;

/// FR4xx gives every interrupt-capable port a vector register that reports and clears one flag in
/// a single read.
pub(crate) const HAS_PORT_IV: bool = true;

/// The analog function. There is no `SEL` encoding for it on this family — [`set_pin_function`]
/// recognises this value and routes the pin to the converter through `SYSCFG2.ADCPCTLx` instead.
pub(crate) const ANALOG_FUNCTION: PinFunction = PinFunction::Alternate3;

/// `WDTCTL`.
pub(crate) const WDTCTL: u16 = 0x01cc;

/// `SYSCFG2`, whose low ten bits are `ADCPCTL0` through `ADCPCTL9`. Setting one disconnects the
/// pin's output driver and input Schmitt trigger and hands it to the converter.
const SYSCFG2: u16 = 0x0164;

/// Base of the port pair `port` belongs to.
///
/// The ports come in 16-bit pairs — P1 and P2 are the low and high halves of PA, and so on — so the
/// odd-numbered port of a pair uses the pair's addresses directly and the even-numbered one adds a
/// byte.
const fn pair_base(port: u8) -> u16 {
    // PA = P1/P2 at 0x0200, PB = P3/P4 at 0x0220, PC = P5/P6 at 0x0240, PD = P7/P8 at 0x0260.
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
///
/// One select bit per pin here, so the digital choices are [`PinFunction::Gpio`] and
/// [`PinFunction::Alternate1`]. [`PinFunction::Alternate3`] is this chip module's own name for the
/// analog function (see [`ANALOG_FUNCTION`]) and is granted through `SYSCFG2` rather than the
/// select register; anything else panics, because the hardware cannot express it.
pub(crate) fn set_pin_function(port: u8, bit: u8, function: PinFunction) {
    const SEL0: u16 = 0x0a;

    let sel0 = match function {
        PinFunction::Gpio => false,
        PinFunction::Alternate1 => true,
        PinFunction::Alternate3 => {
            // A0–A7 are P1.0–P1.7 and A8/A9 are P8.0/P8.1; nothing else reaches the converter.
            let channel = match port {
                0 => bit.trailing_zeros() as u16,
                7 if bit & 0x03 != 0 => 8 + bit.trailing_zeros() as u16,
                _ => panic!("pin has no ADC channel"),
            };
            critical_section::with(|_| {
                // SAFETY: a read-modify-write of SYSCFG2 under a critical section.
                unsafe {
                    let reg = SYSCFG2 as *mut u16;
                    reg.write_volatile(reg.read_volatile() | (1 << channel));
                }
            });
            return;
        }
        PinFunction::Alternate2 => panic!("FR4xx pins have one select bit"),
    };

    let base = pair_base(port) + (port as u16 & 1);
    crate::gpio::modify_reg((base + SEL0) as *mut u8, bit, sel0);
}
