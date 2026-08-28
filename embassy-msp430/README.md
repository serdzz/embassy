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

| Peripheral   | FR2355     | F149        | Async |
|--------------|------------|-------------|-------|
| GPIO         | yes        | yes         | edge wait, on the ports that can interrupt |
| Time driver  | Timer_B0   | Timer_B7    | `embassy-time` |
| Clock system | CS, with the FLL | BCS+  | - |
| Watchdog     | yes        | yes         | - |
| UART         | eUSCI_A    | USART       | yes |
| SPI master   | eUSCI_A/B  | USART       | yes |
| PWM          | Timer_B    | Timer_A/B   | - |
| ADC          | yes        | ADC12       | yes |
| I2C master   | eUSCI_B    | **no hardware** | yes |
| RTC          | yes        | **no hardware** | yes |

The last two are not omissions: the F149's USART does UART and SPI only — I2C arrived on this family
with the F15x/16x — and the device has no real-time clock at all. Bit-banging I2C on two GPIOs is the
usual answer there, and belongs in a driver crate rather than here.

Where a driver exists for both, the API is the same and only the implementation differs, so moving
code between the families is mostly a matter of renaming pins. The ADC is the exception: the FR2xx
converter has selectable resolution and three internal references, the ADC12 is twelve bits with
two, so its configuration genuinely differs and `adc::Config` is not portable between them.

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
