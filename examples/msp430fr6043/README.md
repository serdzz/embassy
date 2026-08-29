# MSP430FR6043 examples

The ultrasonic sensing part, running Embassy.

| Example | What it shows |
| --- | --- |
| `blinky` | The clock, the time driver, the executor and GPIO — the floor |
| `uart` | `eUSCI_A0` echoing characters while an LED keeps blinking |
| `sensors` | ADC12_B, the RTC_C calendar clock and PWM, reporting once a minute and asleep in between |
| `flow` | The ultrasonic front end: ping both ways, correlate, report the difference in flight time |

## What it costs

Linked with `cargo build --release`, measured with `msp430-elf-size`:

| | Flash | RAM | of ~40 kB and 4 kB |
| --- | ---: | ---: | --- |
| `blinky` | 4 680 B | 352 B | 12% and 9% |
| `uart` | 6 146 B | 422 B | 15% and 10% |
| `sensors` | 6 616 B | 414 B | 17% and 10% |
| `flow` | 12 210 B | 1 270 B | 30% and 31% |

So the floor — clock, FRAM wait states, time driver, executor with its low-power sleep, and GPIO —
is about 4.7 kB, and a program using four drivers is about 6.6 kB. A meter that also captures and
correlates is about 12 kB, most of the RAM being the sample buffer: 400 samples at two bytes each.
Roughly **28 kB of FRAM is left** on top of `flow`.

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

## The ultrasonic front end

`embassy_msp430::uss` drives all four modules — `UUPS` for the analog supply, `HSPLL` for the fast
clock, `SAPH_A` for the transducers and the bias, `SDHS` for the converter — and captures a waveform
straight into memory through the converter's own transfer controller. `uss::tof` correlates two
captures and interpolates the peak to get a difference in flight time.

**It is not TI's Ultrasonic Sensing Software Library and will not match it.** That library runs its
correlation on the `LEA` accelerator, tracks the envelope across temperature, compensates for
zero-flow drift, and comes with a calibration workflow against a real flow rig; its accuracy claims
rest on all of that. What is here will show you flow. It will not be right in the third digit, and
nothing here has been near a transducer.

Two smaller gaps, both named in the module docs: the factory drive-strength trims are not loaded
from the `TLV` table, so the output drivers run at reset defaults; and the correlation runs on the
CPU because there is no `LEA` driver, which costs tens of milliseconds per measurement.

## Building and flashing

```sh
cargo build --release
cargo run --release --bin blinky      # mspdebug tilib, so an MSP-FET or the EVM's onboard debugger
```

Needs a nightly toolchain and TI's `msp430-elf-gcc`; see `embassy-msp430`'s README.
