# Embassy on the MSP430F149

Examples for the MSP430F149, on `embassy-executor`'s `platform-msp430`.

| Example  | What it does                                                    |
|----------|-----------------------------------------------------------------|
| `blinky` | Two LEDs blinking at different rates from two tasks.             |
| `button` | Awaits a port interrupt from the key and toggles an LED.         |
| `uart`   | Echoes characters over USART0 at 9600 baud while an LED blinks.  |

These use `embassy-msp430` with its `msp430f149` feature. The F1xx family shares little with the
FR2xx parts below the CPU — BCS+ instead of the CS module, USARTs instead of eUSCIs, the ADC12, and
byte-spaced ports low in the address map rather than pairs of 8-bit halves at 0x0200 — so the
drivers behind the shared API are different code. GPIO, the time driver, the clock system, the
watchdog, UART, SPI, PWM and the ADC are all available; I2C and an RTC are not, because this device
has neither in hardware.

## The PAC

There is no `msp430f149` crate on crates.io, so one is generated and vendored at the repository
root, in `msp430f149-pac/`. See its README for how to reproduce it; it would be better off published
on its own than living here.

## Board

F149 boards are not standardised the way a LaunchPad is. The examples assume the common "minimum
system" layout — LEDs on P1.0 and P1.1, a key on P1.4 — and that is the first thing to check against
your own schematic.

Two things the F1xx will catch you out on:

- **The ports have no pull resistors.** Those arrived with the F2xx family, so the key needs an
  external pull-up, and `Input::new` is asked for `Pull::None`. Asking for a pull on this family
  panics rather than quietly doing nothing.
- **ACLK comes from the LFXT1 crystal, and there is no internal low-frequency oscillator.** The time
  driver counts ACLK, so the board needs its 32768 Hz watch crystal fitted or nothing will tick. A
  watch crystal also takes up to a second to start, so time runs slow for a moment after reset.

## Toolchain

As for any MSP430 target: a nightly compiler with `rust-src` (`core` is built from source), and TI's
`msp430-elf-gcc` for linking and for the `__mspabi_*` helpers that arithmetic wider than a shift
lowers to.

```sh
cargo build --release --bin blinky
cargo run --release --bin blinky
```

Unlike the FR2xx parts, the F149 *is* in mspdebug's own device table, so the drivers that talk to a
probe directly all work: `uif` for an MSP-FET430UIF, `olimex` for the Olimex tools, `rf2500` for a
Launchpad. `.cargo/config.toml` uses `uif`; change it there.

## Simulating

The F149 is also the one MSP430 in this repository that mspdebug can simulate properly. Its ports
sit exactly where mspdebug's GPIO model expects them, its Timer_B is inside the simulated peripheral
window, and its interrupt vectors fit the sixteen the simulator can dispatch. So the LEDs really do
change state and the key really does raise an interrupt:

```sh
mspdebug -n sim
```

```
(mspdebug) simio add timer tb 7
(mspdebug) simio config tb base 0x0180
(mspdebug) simio config tb type B
(mspdebug) simio config tb irq0 13
(mspdebug) simio config tb irq1 12
(mspdebug) simio config tb iv 0x011e
(mspdebug) simio add gpio p1
(mspdebug) simio config p1 base 0x0020
(mspdebug) simio config p1 irq 4
(mspdebug) simio config p1 verbose
(mspdebug) prog target/msp430-none-elf/release/blinky
(mspdebug) step 12000000
gpio: state change on p1: ---- ---H
gpio: state change on p1: ---- --H-
gpio: state change on p1: ---- ---l
...
```

For `button`, `simio config p1 set 4 0` presses the key and `set 4 1` releases it.
