# Embassy on the MSP430FR4133 LaunchPad

Examples for the [MSP-EXP430FR4133](https://www.ti.com/tool/MSP-EXP430FR4133) LaunchPad, built on
`embassy-msp430` and `embassy-executor`'s `platform-msp430`.

| Example  | What it does                                            |
|----------|---------------------------------------------------------|
| `blinky` | LED1 (P1.0) and LED2 (P4.0) blinking from two tasks.    |

The `embassy-time` driver runs on Timer0_A3 (`time-driver-ta0`): the FR4133 has no Timer_B, which
is the timer the other MSP430 examples use.

## Toolchain

`msp430-none-elf` is a tier 3 target, so you need:

- A nightly compiler with `rust-src` (`rust-toolchain.toml` pins this). `core` is built from source
  via `-Zbuild-std`, configured in `.cargo/config.toml`.
- [TI's `msp430-gcc`](https://www.ti.com/tool/MSP430-GCC-OPENSOURCE), for `msp430-elf-gcc`. It is
  the linker, and it supplies `libgcc` — MSP430 has neither a divider nor a multiplier, so
  arithmetic wider than a shift lowers to `__mspabi_*` calls that live there.

  `rust-lld` is not a substitute: it cannot relocate `R_MSP430_SYM_DIFF`, which `msp430-rt`'s
  prebuilt startup object uses.

## Flashing

[`mspdebug`](https://dlbeer.co.nz/mspdebug/) does the programming, wired up as the cargo runner in
`.cargo/config.toml`:

```sh
cargo build --release --bin blinky
cargo run --release --bin blinky
```

It uses mspdebug's `tilib` driver, which talks to the probe through TI's `libmsp430` — the same
setup as the FR2355 examples; see that README for the details. The LaunchPad's on-board eZ-FET and
a separate MSP-FET or MSP-FET430UIF both work.
