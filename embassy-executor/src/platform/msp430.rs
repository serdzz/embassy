#[cfg(feature = "executor-interrupt")]
compile_error!("`executor-interrupt` is not supported with `platform-msp430`.");

/// Define an interrupt handler that wakes the thread-mode executor up when it returns.
///
/// On MSP430 an interrupt does *not* by itself take the CPU out of a low-power mode: the status
/// register is pushed on interrupt entry and restored by `reti`, so a core that went to sleep with
/// `CPUOFF` set goes right back to sleep when the handler returns. To resume, a handler has to clear
/// the low-power-mode bits in the *stacked* status register.
///
/// This macro takes care of that. It emits the vector as a naked trampoline that clears
/// `SCG1 | SCG0 | OSCOFF | CPUOFF` in the stacked status register and then tail-jumps into the
/// handler body, so [`Executor::run`] regains control as soon as the handler returns.
///
/// Any interrupt that wakes a task (directly, or through a [`Waker`](core::task::Waker)) must be
/// declared with this macro instead of [`msp430_rt::interrupt`], otherwise its wakeup is lost until
/// something else happens to wake the CPU.
///
/// The handler name must match the vector name provided by the `device.x` of your PAC, e.g.
/// `TIMER0_B0` for `msp430fr2355`.
///
/// # Example
///
/// ```ignore
/// #![feature(abi_msp430_interrupt, asm_experimental_arch)]
///
/// embassy_executor::msp430_interrupt! {
///     unsafe fn TIMER0_B0() {
///         // ... clear the interrupt flag, wake whoever is waiting ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! msp430_interrupt {
    ($(
        $(#[$attr:meta])*
        unsafe fn $name:ident() $body:block
    )*) => {$(
        const _: () = {
            #[unsafe(naked)]
            #[unsafe(no_mangle)]
            #[allow(non_snake_case)]
            $(#[$attr])*
            unsafe extern "msp430-interrupt" fn $name() {
                ::core::arch::naked_asm!(
                    // Clear SCG1 | SCG0 | OSCOFF | CPUOFF in the status register that `reti` will
                    // restore, so the CPU stays awake once this handler returns. On interrupt entry
                    // the stacked SR sits at the top of the stack, and nothing has been pushed on
                    // top of it yet because this trampoline is naked.
                    "bic #0xf0, 0(r1)",
                    // Tail-jump into the real handler (`r0` is PC). It has the `msp430-interrupt`
                    // ABI too, so it saves/restores the registers it clobbers and owns the `reti`.
                    "mov #{handler}, r0",
                    handler = sym __handler,
                );
            }

            unsafe extern "msp430-interrupt" fn __handler() $body
        };
    )*};
}

#[cfg(feature = "executor-thread")]
pub use thread::*;
#[cfg(feature = "executor-thread")]
mod thread {
    use core::arch::asm;
    use core::marker::PhantomData;

    pub use embassy_executor_macros::main_msp430 as main;
    use portable_atomic::{AtomicBool, AtomicU8, Ordering};

    use crate::{Spawner, raw};

    /// Global interrupt enable bit of the status register.
    const GIE: u16 = 0x0008;
    /// Turns off the CPU.
    const CPUOFF: u16 = 0x0010;
    /// Turns off the LFXT1 crystal oscillator.
    const OSCOFF: u16 = 0x0020;
    /// Turns off the DCO dc generator.
    const SCG0: u16 = 0x0040;
    /// Turns off SMCLK.
    const SCG1: u16 = 0x0080;

    /// Low-power mode the thread-mode executor enters when it runs out of work.
    ///
    /// Deeper modes save more power but keep fewer clocks running, so they also limit which
    /// peripherals can still wake the CPU. Pick the deepest mode that keeps the clock feeding your
    /// [`embassy-time`](https://docs.rs/embassy-time) driver alive: e.g. `Lpm3` for an ACLK-driven
    /// timer, `Lpm0` for an SMCLK-driven one.
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum LowPowerMode {
        /// Don't sleep at all, spin instead. Burns power, but keeps every clock running.
        Active = 0,
        /// CPU and MCLK off. ACLK and SMCLK keep running.
        Lpm0 = 1,
        /// CPU, MCLK and the DCO dc generator off. ACLK and SMCLK keep running.
        Lpm1 = 2,
        /// CPU, MCLK and SMCLK off. ACLK keeps running, the DCO dc generator stays on.
        Lpm2 = 3,
        /// CPU, MCLK, SMCLK and the DCO dc generator off. ACLK keeps running.
        Lpm3 = 4,
        /// Everything off, including the crystal oscillator. Only an asynchronous interrupt
        /// (a GPIO edge, a reset or an NMI) can wake the CPU up again.
        Lpm4 = 5,
    }

    static SIGNAL_WORK_THREAD_MODE: AtomicBool = AtomicBool::new(false);
    static LOW_POWER_MODE: AtomicU8 = AtomicU8::new(LowPowerMode::Lpm0 as u8);

    /// Set the low-power mode the executor sleeps in when it has nothing left to poll.
    ///
    /// Defaults to [`LowPowerMode::Lpm0`]. It can be changed at any time, including from a task, to
    /// follow whichever peripherals are currently in use. The new mode takes effect the next time
    /// the executor goes to sleep.
    pub fn set_low_power_mode(mode: LowPowerMode) {
        LOW_POWER_MODE.store(mode as u8, Ordering::Relaxed);
    }

    /// Get the low-power mode the executor sleeps in. See [`set_low_power_mode`].
    pub fn low_power_mode() -> LowPowerMode {
        match LOW_POWER_MODE.load(Ordering::Relaxed) {
            0 => LowPowerMode::Active,
            1 => LowPowerMode::Lpm0,
            2 => LowPowerMode::Lpm1,
            3 => LowPowerMode::Lpm2,
            4 => LowPowerMode::Lpm3,
            _ => LowPowerMode::Lpm4,
        }
    }

    #[unsafe(export_name = "__pender")]
    fn __pender(_context: *mut ()) {
        SIGNAL_WORK_THREAD_MODE.store(true, Ordering::SeqCst);
    }

    /// Enable interrupts and go to sleep, as a single instruction.
    ///
    /// `bis` on the status register commits both bits at once, so no interrupt can be taken between
    /// re-enabling them and entering the low-power mode: a pending wakeup can't be missed.
    ///
    /// # Safety
    ///
    /// Interrupts are enabled, so this must not be called while holding a `CriticalSection`.
    #[inline(always)]
    unsafe fn sleep() {
        macro_rules! enter {
            ($bits:expr) => {
                // No `nomem`: the interrupt handler that wakes us up runs, and writes memory,
                // "inside" this instruction as far as the compiler is concerned.
                asm!("bis #{bits}, r2", bits = const ($bits | GIE), options(nostack))
            };
        }

        match low_power_mode() {
            LowPowerMode::Active => unsafe { msp430::interrupt::enable() },
            LowPowerMode::Lpm0 => enter!(CPUOFF),
            LowPowerMode::Lpm1 => enter!(SCG0 | CPUOFF),
            LowPowerMode::Lpm2 => enter!(SCG1 | CPUOFF),
            LowPowerMode::Lpm3 => enter!(SCG1 | SCG0 | CPUOFF),
            LowPowerMode::Lpm4 => enter!(SCG1 | SCG0 | OSCOFF | CPUOFF),
        }
    }

    /// MSP430 Executor
    pub struct Executor {
        inner: raw::Executor,
        not_send: PhantomData<*mut ()>,
    }

    impl Executor {
        /// Create a new Executor.
        pub fn new() -> Self {
            Self {
                inner: raw::Executor::new(core::ptr::null_mut()),
                not_send: PhantomData,
            }
        }

        /// Run the executor.
        ///
        /// The `init` closure is called with a [`Spawner`] that spawns tasks on
        /// this executor. Use it to spawn the initial task(s). After `init` returns,
        /// the executor starts running the tasks.
        ///
        /// To spawn more tasks later, you may keep copies of the [`Spawner`] (it is `Copy`),
        /// for example by passing it as an argument to the initial tasks.
        ///
        /// This function requires `&'static mut self`. This means you have to store the
        /// Executor instance in a place where it'll live forever and grants you mutable
        /// access. There's a few ways to do this:
        ///
        /// - a [StaticCell](https://docs.rs/static_cell/latest/static_cell/) (safe)
        /// - a `static mut` (unsafe)
        /// - a local variable in a function you know never returns (like `fn main() -> !`), upgrading its lifetime with `transmute`. (unsafe)
        ///
        /// This function never returns.
        ///
        /// # Waking up from sleep
        ///
        /// Every interrupt that can wake a task up must be declared with
        /// [`msp430_interrupt!`](crate::msp430_interrupt), otherwise it will not take the CPU out
        /// of its low-power mode and the wakeup will be delayed until something else does.
        pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {
            init(self.inner.spawner());

            loop {
                unsafe {
                    msp430::interrupt::disable();
                    if SIGNAL_WORK_THREAD_MODE.swap(false, Ordering::SeqCst) {
                        msp430::interrupt::enable();
                        self.inner.poll();
                    } else {
                        sleep();
                    }
                }
            }
        }
    }
}
