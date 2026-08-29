# embassy-msp430

An [Embassy](https://embassy.dev) Hardware Abstraction Layer (HAL) for TI MSP430 microcontrollers.

The HAL implements both blocking and async APIs for many peripherals. Where an async API is
available, it is implemented on top of the interrupt the peripheral already raises, so the
executor can stay in a low-power mode while it waits.

MSP430 is a tier 3 Rust target, so this crate needs a nightly compiler, `-Zbuild-std=core`, and
TI's `msp430-elf-gcc` for linking. See `examples/msp430fr2355` for a working setup.

## Supported chips

Enable exactly one:

| Feature          | Device       | Family      | Reachable code space |
|------------------|--------------|-------------|----------------------|
| `msp430fr2355`   | MSP430FR2355 | FR2xx, FRAM | 32 kB, all of it |
| `msp430f149`     | MSP430F149   | F1xx, flash | 60 kB, all of it |
| `msp430f2618`    | MSP430F2618  | F2xx, flash | ~51 kB of 116 |
| `msp430fr6043`   | MSP430FR6043 | FR6xx, FRAM | ~40 kB of 64 |

The families share the CPU and little else, so not every driver exists for all of them. What differs
is kept in `src/chip/`; adding a device is a file there plus a feature.

The last two are MSP430X parts whose memory runs past 0xFFFF. Rust's `msp430-none-elf` target is
16-bit throughout and has no 20-bit addressing, so the upper part is unreachable and the linker
script has to stop at the vector table. Buying a bigger part in those families does not buy more
room until that changes.

## Peripherals

| Peripheral   | FR2355     | F149        | F2618       | FR6043      | Async |
|--------------|------------|-------------|-------------|-------------|-------|
| GPIO         | yes        | yes         | yes         | yes         | edge wait, on the ports that can interrupt |
| Time driver  | Timer_B0   | Timer_B7    | Timer_B7    | Timer_B0    | `embassy-time` |
| Clock system | CS, with the FLL | BCS+  | BCS+, calibrated | CS, fixed DCO | - |
| Watchdog     | yes        | yes         | yes         | yes         | - |
| UART         | eUSCI_A    | USART       | not yet     | eUSCI_A ×4  | yes |
| SPI master   | eUSCI_A/B  | USART       | not yet     | eUSCI_A/B   | yes |
| I2C master   | eUSCI_B    | **no hardware** | not yet | eUSCI_B ×2  | yes |
| PWM          | Timer_B    | Timer_A/B   | not yet     | Timer_A ×5  | - |
| ADC          | yes        | ADC12       | not yet     | ADC12_B     | yes |
| RTC          | yes        | **no hardware** | **no hardware** | RTC_C  | yes |

"Not yet" is work not done. **No hardware** is not an omission: the F149's USART does UART and SPI
only — I2C arrived on this family with the F15x/16x — and neither it nor the F2618 has a real-time
clock at all. Bit-banging I2C on two GPIOs is the usual answer there, and belongs in a driver crate
rather than here.

The FR6043's ultrasonic front end — `SAPH_A`, `SDHS`, `UUPS`, `HSPLL` and the `LEA` accelerator — is
not wrapped. Those are peripheral singletons with no code behind them; see
`examples/msp430fr6043/README.md`.

On devices with more peripherals than convenient pins, which eUSCI a pin carries is not the same
alternate function on every pin that can carry it. The pin traits carry that per pin, so the
compiler picks the right `PxSEL` value from the pin you hand it.

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
