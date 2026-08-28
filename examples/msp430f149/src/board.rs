//! The board-specific half of these examples.
//!
//! F149 boards are not standardised the way a LaunchPad is, so **check these pins against your own
//! schematic** before wiring anything up. They match the common "minimum system" boards that put
//! two LEDs on P1.0 and P1.1 and a key on P1.4.
//!
//! Two F1xx facts worth knowing:
//!
//! - The ports have no pull resistors at all. Those arrived with the F2xx family, so a button here
//!   needs an external pull-up; there is nothing to enable in software.
//! - ACLK comes from the LFXT1 crystal, and there is no internal low-frequency oscillator to fall
//!   back on. The time driver counts ACLK, so the board needs its 32768 Hz watch crystal fitted. A
//!   watch crystal also takes up to a second to start, so time runs slow for a moment after reset.

use embassy_sync::waitqueue::AtomicWaker;

use crate::pac;

const LED1_BIT: u8 = 1 << 0;
const LED2_BIT: u8 = 1 << 1;
const BUTTON_BIT: u8 = 1 << 4;

static BUTTON_WAKER: AtomicWaker = AtomicWaker::new();

/// LED1, on P1.0.
pub struct Led1(());

/// LED2, on P1.1.
pub struct Led2(());

/// The key on P1.4. Active low, pulled up externally.
pub struct Button(());

/// What the examples use.
pub struct Board {
    pub led1: Led1,
    pub led2: Led2,
    pub button: Button,
}

fn port1() -> pac::Port1_2 {
    // SAFETY: `init` takes the port once and never hands it out again; the handles below own
    // distinct bits of it, and the port ISR only touches the button's.
    unsafe { pac::Port1_2::steal() }
}

macro_rules! impl_led {
    ($name:ident, $bit:expr) => {
        impl $name {
            /// Drive the LED on or off.
            pub fn set(&mut self, on: bool) {
                port1()
                    .p1out()
                    .modify(|r, w| unsafe { w.bits(if on { r.bits() | $bit } else { r.bits() & !$bit }) });
            }

            /// Flip the LED.
            pub fn toggle(&mut self) {
                port1().p1out().modify(|r, w| unsafe { w.bits(r.bits() ^ $bit) });
            }
        }
    };
}

impl_led!(Led1, LED1_BIT);
impl_led!(Led2, LED2_BIT);

impl Button {
    /// `true` while the key is held down.
    pub fn is_pressed(&self) -> bool {
        port1().p1in().read().bits() & BUTTON_BIT == 0
    }

    /// Wait for the next press.
    ///
    /// Not debounced: one press can produce several edges.
    pub async fn wait_for_press(&mut self) {
        critical_section::with(|_| {
            let p = port1();
            // Falling edge, since the key pulls the pin down.
            p.p1ies().modify(|r, w| unsafe { w.bits(r.bits() | BUTTON_BIT) });
            // Choosing an edge can set the flag by itself, so clear it after choosing.
            p.p1ifg().modify(|r, w| unsafe { w.bits(r.bits() & !BUTTON_BIT) });
            p.p1ie().modify(|r, w| unsafe { w.bits(r.bits() | BUTTON_BIT) });
        });

        core::future::poll_fn(|cx| {
            BUTTON_WAKER.register(cx.waker());
            critical_section::with(|_| {
                // The handler masks the pin, so "has it fired" is "is it still enabled".
                if port1().p1ie().read().bits() & BUTTON_BIT == 0 {
                    core::task::Poll::Ready(())
                } else {
                    core::task::Poll::Pending
                }
            })
        })
        .await;

        critical_section::with(|_| {
            port1().p1ifg().modify(|r, w| unsafe { w.bits(r.bits() & !BUTTON_BIT) });
        });
    }
}

/// Bring the board up: stop the watchdog, set the pin directions and start the time driver.
///
/// Panics if called more than once.
pub fn init() -> Board {
    let p = pac::Peripherals::take().expect("board::init called twice");

    // The watchdog runs out of reset and would reset the chip after about 32 ms. Its register is
    // password protected, so it is written whole rather than read-modify-written.
    p.watchdog_timer.wdtctl().write(|w| unsafe { w.bits(0x5A80) }); // WDTPW | WDTHOLD

    // LEDs off and driven, button an input. Nothing else needs configuring: BCS+ comes out of reset
    // with MCLK on the DCO and ACLK on LFXT1, which is exactly what the time driver wants.
    let port = p.port_1_2;
    port.p1out()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(LED1_BIT | LED2_BIT)) });
    port.p1dir()
        .modify(|r, w| unsafe { w.bits((r.bits() | LED1_BIT | LED2_BIT) & !BUTTON_BIT) });
    port.p1ie().modify(|r, w| unsafe { w.bits(r.bits() & !BUTTON_BIT) });

    crate::time_driver::init(p.timer_b7);

    Board {
        led1: Led1(()),
        led2: Led2(()),
        button: Button(()),
    }
}

embassy_executor::msp430_interrupt! {
    /// Port 1 edge: the key was pressed.
    unsafe fn PORT1() {
        // Mask the pin: that is how the future learns its edge arrived, and it stops the interrupt
        // firing again before the task has had a chance to run.
        port1().p1ie().modify(|r, w| unsafe { w.bits(r.bits() & !BUTTON_BIT) });
        BUTTON_WAKER.wake();
    }
}
