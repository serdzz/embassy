# embassy-msp430

An [Embassy](https://embassy.dev) Hardware Abstraction Layer (HAL) for TI MSP430 microcontrollers.

The HAL implements both blocking and async APIs for many peripherals. Where an async API is
available, it is implemented on top of the interrupt the peripheral already raises, so the
executor can stay in a low-power mode while it waits.

MSP430 is a tier 3 Rust target, so this crate needs a nightly compiler, `-Zbuild-std=core`, and
TI's `msp430-elf-gcc` for linking. See `examples/msp430fr2355` for a working setup.

## Supported chips

- MSP430FR2355

## Peripherals

| Peripheral | Blocking | Async |
|------------|----------|-------|
| GPIO       | yes      | edge wait on P1..P4 |
| UART (eUSCI_A) | yes  | yes   |
| SPI master (eUSCI_A/B) | yes | yes |
| I2C master (eUSCI_B) | yes | yes |
| ADC        | yes      | yes   |
| PWM (Timer_B) | yes   | -     |
| RTC        | yes      | yes   |
| Clock system (CS) | yes | -   |
| Watchdog   | yes      | -     |
| Timer_B0   | -        | `embassy-time` driver |

Input capture and the eCOMP/SAC analog blocks are not wrapped yet; reach for `pac`, which this
crate re-exports, until they are.

One eUSCI module can be a UART, an SPI or an I2C, but only one at a time. Which one is decided by
the driver you hand the peripheral singleton to, so the compiler enforces it.

## Waking up from sleep

An MSP430 interrupt does not by itself take the CPU out of a low-power mode: the status register is
stacked on entry and restored by `reti`. Every interrupt handler that wakes a task must therefore
clear the low-power bits in the *stacked* status register, which is what
`embassy_executor::msp430_interrupt!` generates. The handlers in this crate already do; if you add
your own, declare them with that macro rather than with `#[msp430_rt::interrupt]`.
