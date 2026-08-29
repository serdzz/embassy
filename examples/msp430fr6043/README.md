# MSP430FR6043 examples

The ultrasonic sensing part, running Embassy.

| Example | What it shows |
| --- | --- |
| `blinky` | The clock, the time driver, the executor and GPIO — the floor |
| `uart` | `eUSCI_A0` echoing characters while an LED keeps blinking |
| `sensors` | ADC12_B, the RTC_C calendar clock and PWM, reporting once a minute and asleep in between |

## What it costs

Linked with `cargo build --release`, measured with `msp430-elf-size`:

| | Flash | RAM | of ~40 kB and 4 kB |
| --- | ---: | ---: | --- |
| `blinky` | 4 524 B | 348 B | 11% and 8% |
| `uart` | 5 990 B | 418 B | 15% and 10% |
| `sensors` | 6 460 B | 410 B | 16% and 10% |

So the floor — clock, FRAM wait states, time driver, executor with its low-power sleep, and GPIO —
is about 4.5 kB, and a program using four drivers is about 6.5 kB. Roughly **34 kB of FRAM is left**
for an application.

Whether that is enough depends entirely on what the application is. For a flow meter it has to hold
TI's ultrasonic sensing library, the metrology, the display and the comms — worth measuring before
committing to the part, because of the ceiling described next.

Note that every interrupt handler in the HAL is linked into every binary whether its driver is used
or not: the vector table is a static array of function pointers, so the linker cannot drop them.
That is why `blinky` grew when the ADC and RTC drivers were added.

## The ceiling

The device has 64 kB of FRAM. About 40 kB of it is reachable: the rest sits above 0xFFFF and needs
the 20-bit addressing of the MSP430X, which Rust's `msp430-none-elf` target does not have. The same
is true of the bigger parts in the family — the FR6047 has 256 kB of FRAM and roughly 48 kB of it is
reachable — so buying more FRAM does not buy more room until Rust learns 20-bit addressing.

## What is here

GPIO, the `embassy-time` driver on Timer_B0, the clock system with its FRAM wait states, the
watchdog, UART, SPI and I2C across all six eUSCI instances, PWM on the five Timer_A instances, the
ADC12_B converter and the RTC_C calendar clock.

### One thing to check before trusting the pin assignments

A pin on this device carries up to three peripheral functions, chosen by `PxSEL1:PxSEL0`, and which
one a given eUSCI is differs from pin to pin — `UCA3TXD` is the second alternate on P2.0 and the
third on P4.2. The HAL carries that per pin, and every one of those values is quoted in the source
beside the pinout string it came from.

Those values are **derived from the order the functions are listed in the datasheet's package
pinout**, which is how this family's datasheets are written. The table that states the encoding
outright — Table 7-1 — is a graphic in the PDF and could not be read out of it, so none of these has
been confirmed against it, and none has run on hardware. A wrong alternate does not fail loudly: the
peripheral simply never reaches the pin. Worth an hour with the datasheet before the first board.

## What is not here

Any driver for the ultrasonic front end. `SAPH_A`, `SDHS`, `UUPS`, `HSPLL` and `LEA` are in the PAC
and reachable, and they are declared as peripheral singletons so nothing else claims them, but there
is no code behind them.

That is not an oversight, and it is not a small gap. The registers are the easy part; what makes an
ultrasonic meter work is the excitation sequence, start-of-frame detection, capturing a thousand
samples per direction, and the cross-correlation and interpolation that turn those into a
time-of-flight difference measured in picoseconds — plus temperature compensation and calibration
against a real flow rig. That is TI's USS library, it runs on the LEA accelerator, and reproducing
it is a signal-processing project rather than a driver.

The realistic options are to link TI's library and keep Rust outside it, or to use an external
time-to-digital front end on a part with better Rust support.

## Building and flashing

```sh
cargo build --release
cargo run --release --bin blinky      # mspdebug tilib, so an MSP-FET or the EVM's onboard debugger
```

Needs a nightly toolchain and TI's `msp430-elf-gcc`; see `embassy-msp430`'s README.
