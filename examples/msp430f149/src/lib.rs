#![no_std]
// MSP430 is a tier 3 target: the interrupt ABI and inline assembly are still unstable there.
#![feature(abi_msp430_interrupt, asm_experimental_arch)]

//! Embassy on the MSP430F149.
//!
//! The F1xx family is a generation older than the FR2xx parts `embassy-msp430` targets, and shares
//! almost nothing with them below the CPU: the clock system is BCS+ rather than CS, the serial
//! peripherals are USARTs rather than eUSCIs, the converter is the ADC12, and the ports are
//! byte-spaced low in the address map instead of paired into 16-bit halves. So these examples do
//! not use the HAL; they drive the peripheral access crate directly, the way the FR2355 examples
//! did before it existed. What they do share is `embassy-executor`'s MSP430 platform, which is the
//! point of having them.

pub use msp430f149 as pac;
use panic_msp430 as _;

pub mod board;
pub mod time_driver;
