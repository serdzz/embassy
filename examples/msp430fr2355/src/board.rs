//! The bits of the MSP-EXP430FR2355 LaunchPad the examples use.
//!
//! This is deliberately tiny: there is no `embassy-msp430` HAL yet, so the examples poke the PAC
//! directly and this module just keeps the pin numbers in one place.
//!
//! | Function | Pin  |
//! |----------|------|
//! | LED1     | P1.0 |
//! | LED2     | P6.6 |
//! | S1       | P4.1 |

use core::cell::Cell;

use critical_section::Mutex;
use embassy_sync::waitqueue::AtomicWaker;
use msp430fr2355::{P1, P4, P6, Peripherals, Pmm, WdtA};

const LED1_BIT: u8 = 1 << 0;
const LED2_BIT: u8 = 1 << 6;
const BUTTON_BIT: u8 = 1 << 1;

static BUTTON_WAKER: AtomicWaker = AtomicWaker::new();
/// Set by the `PORT4` handler, consumed by [`Button::wait_for_press`].
static BUTTON_PRESSED: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

/// LED1 (red), on P1.0.
pub struct Led1(());

/// LED2 (green), on P6.6.
pub struct Led2(());

/// Button S1, on P4.1. Active low, with the internal pull-up enabled.
pub struct Button(());

/// The peripherals the examples use.
pub struct Board {
    pub led1: Led1,
    pub led2: Led2,
    pub button: Button,
}

impl Led1 {
    pub fn set(&mut self, on: bool) {
        let p1 = unsafe { P1::steal() };
        p1.p1out()
            .modify(|r, w| unsafe { w.bits(if on { r.bits() | LED1_BIT } else { r.bits() & !LED1_BIT }) });
    }

    pub fn toggle(&mut self) {
        let p1 = unsafe { P1::steal() };
        p1.p1out().modify(|r, w| unsafe { w.bits(r.bits() ^ LED1_BIT) });
    }
}

impl Led2 {
    pub fn set(&mut self, on: bool) {
        let p6 = unsafe { P6::steal() };
        p6.p6out()
            .modify(|r, w| unsafe { w.bits(if on { r.bits() | LED2_BIT } else { r.bits() & !LED2_BIT }) });
    }

    pub fn toggle(&mut self) {
        let p6 = unsafe { P6::steal() };
        p6.p6out().modify(|r, w| unsafe { w.bits(r.bits() ^ LED2_BIT) });
    }
}

impl Button {
    /// `true` while the button is held down.
    pub fn is_pressed(&self) -> bool {
        let p4 = unsafe { P4::steal() };
        p4.p4in().read().bits() & BUTTON_BIT == 0
    }

    /// Wait for the next press.
    ///
    /// The button is not debounced: a single press can produce a handful of edges, so a press that
    /// arrives while nothing is awaiting is remembered and returned by the next call.
    pub async fn wait_for_press(&mut self) {
        core::future::poll_fn(|cx| {
            // Register first, so a press landing between the check below and the next poll still
            // wakes us.
            BUTTON_WAKER.register(cx.waker());

            critical_section::with(|cs| {
                if BUTTON_PRESSED.borrow(cs).replace(false) {
                    core::task::Poll::Ready(())
                } else {
                    core::task::Poll::Pending
                }
            })
        })
        .await
    }
}

/// Bring the board up: stop the watchdog, unlock the I/O, configure the LEDs and the button, and
/// start the [`embassy-time`](embassy_time) driver.
///
/// Panics if called more than once.
pub fn init() -> Board {
    let p = Peripherals::take().expect("board::init called twice");

    // The watchdog is running out of reset and would reset us after ~32 ms.
    let wdt: WdtA = p.wdt_a;
    wdt.wdtctl().write(|w| unsafe { w.bits(0x5A80) }); // WDTPW | WDTHOLD

    // Out of reset the FRAM parts keep every pin in the high-impedance state they had in LPM4.
    // Nothing on a port takes effect until LOCKLPM5 is cleared.
    let pmm: Pmm = p.pmm;
    pmm.pm5ctl0().modify(|_, w| w.locklpm5().locklpm5_0());

    // LED1 on P1.0, driven low.
    let p1: P1 = p.p1;
    p1.p1out().modify(|r, w| unsafe { w.bits(r.bits() & !LED1_BIT) });
    p1.p1dir().modify(|r, w| unsafe { w.bits(r.bits() | LED1_BIT) });

    // LED2 on P6.6, driven low.
    let p6: P6 = p.p6;
    p6.p6out().modify(|r, w| unsafe { w.bits(r.bits() & !LED2_BIT) });
    p6.p6dir().modify(|r, w| unsafe { w.bits(r.bits() | LED2_BIT) });

    // S1 on P4.1: input, pull-up (with the pin an input, P4OUT picks pull-up over pull-down),
    // interrupt on the falling edge, i.e. on press.
    let p4: P4 = p.p4;
    p4.p4dir().modify(|r, w| unsafe { w.bits(r.bits() & !BUTTON_BIT) });
    p4.p4out().modify(|r, w| unsafe { w.bits(r.bits() | BUTTON_BIT) });
    p4.p4ren().modify(|r, w| unsafe { w.bits(r.bits() | BUTTON_BIT) });
    p4.p4ies().modify(|r, w| unsafe { w.bits(r.bits() | BUTTON_BIT) });
    p4.p4ifg().modify(|r, w| unsafe { w.bits(r.bits() & !BUTTON_BIT) });
    p4.p4ie().modify(|r, w| unsafe { w.bits(r.bits() | BUTTON_BIT) });

    crate::time_driver::init();

    Board {
        led1: Led1(()),
        led2: Led2(()),
        button: Button(()),
    }
}

embassy_executor::msp430_interrupt! {
    /// Port 4 edge: the button was pressed.
    unsafe fn PORT4() {
        let p4 = unsafe { P4::steal() };
        // Reading P4IV clears the flag it reports, so the interrupt doesn't re-fire immediately.
        let _ = p4.p4iv().read().bits();
        critical_section::with(|cs| BUTTON_PRESSED.borrow(cs).set(true));
        BUTTON_WAKER.wake();
    }
}
