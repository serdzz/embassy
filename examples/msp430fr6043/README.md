# MSP430FR6043 examples

The ultrasonic sensing part, running Embassy. One example so far — `blinky` — and its job is less to
blink an LED than to answer a question: **how much of this device is left after the basics?**

## What it costs

Built with `cargo build --release`, and measured with `msp430-elf-size`:

| | | of what is available |
| --- | ---: | --- |
| Code and constants | 3 946 B | ~40 kB of reachable FRAM |
| RAM (`.data` + `.bss`) | 292 B | 4 kB |

That is the clock system, the FRAM wait states that go with it, the `embassy-time` driver on
Timer_B0, the executor with its low-power sleep, and GPIO. Roughly **36 kB of FRAM and 3.8 kB of RAM
are left** for everything else.

Whether that is enough depends entirely on what "everything else" is. For a flow meter it has to
hold TI's ultrasonic sensing library, the metrology, the display and the comms — and that is worth
measuring before committing to the part, because of the ceiling described next.

## The ceiling

The device has 64 kB of FRAM. About 40 kB of it is reachable: the rest sits above 0xFFFF and needs
the 20-bit addressing of the MSP430X, which Rust's `msp430-none-elf` target does not have. The same
is true of the bigger parts in the family — the FR6047 has 256 kB of FRAM and roughly 48 kB of it is
reachable — so buying more FRAM does not buy more room until Rust learns 20-bit addressing.

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
