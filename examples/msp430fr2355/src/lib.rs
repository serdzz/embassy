#![no_std]
// MSP430 is a tier 3 target: the interrupt ABI and inline assembly are still unstable there.
#![feature(abi_msp430_interrupt, asm_experimental_arch)]

pub mod board;
pub mod time_driver;

/// Stop dead on panic. There is no debug channel wired up on the LaunchPad by default, so there is
/// nothing useful to report; halting at least keeps the failure visible under a debugger.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    msp430::interrupt::disable();
    loop {
        msp430::asm::barrier();
    }
}
