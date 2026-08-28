# embassy-msp430

An [Embassy](https://embassy.dev) Hardware Abstraction Layer (HAL) for TI MSP430 microcontrollers.

The HAL implements both blocking and async APIs for many peripherals. Where an async API is
available, it is implemented on top of the interrupt the peripheral already raises, so the
executor can stay in a low-power mode while it waits.

MSP430 is a tier 3 Rust target, so this crate needs a nightly compiler, `-Zbuild-std=core`, and
TI's `msp430-elf-gcc` for linking. See `examples/msp430fr2355` for a working setup.

## Supported chips

Enable exactly one:

| Feature          | Device      | Family |
|------------------|-------------|--------|
| `msp430fr2355`   | MSP430FR2355 | FR2xx, FRAM |
| `msp430f149`     | MSP430F149   | F1xx, flash |

The two families share the CPU and little else, so not every driver exists for both. What differs is
kept in `src/chip/`; adding a device is a file there plus a feature.

## Peripherals

| Peripheral        | FR2355 | F149 | Async |
|-------------------|--------|------|-------|
| GPIO              | yes    | yes  | edge wait, on the ports that can interrupt |
| Time driver (Timer_B) | yes | yes | `embassy-time` |
| Clock system      | CS, with the FLL | BCS+ | - |
| Watchdog          | yes    | yes  | - |
| UART              | eUSCI_A | -   | yes |
| SPI master        | eUSCI_A/B | - | yes |
| I2C master        | eUSCI_B | -   | yes |
| ADC               | yes    | -    | yes |
| PWM (Timer_B)     | yes    | -    | - |
| RTC               | yes    | -    | yes |

The F149's USARTs and ADC12 have nothing in common with the FR2xx eUSCI and ADC beyond the job they
do, so they need drivers of their own rather than a widened version of these; they are not written
yet.

Two differences the GPIO driver surfaces rather than papers over: F1xx has no pull resistors at all,
so asking for one panics instead of quietly doing nothing, and it has interrupts on P1 and P2 only
rather than P1 through P4.

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
