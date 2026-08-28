# Embassy on the MSP430FR2355 LaunchPad

Examples for the [MSP-EXP430FR2355](https://www.ti.com/tool/MSP-EXP430FR2355) LaunchPad, running on
`embassy-executor`'s `platform-msp430`.

There is no `embassy-msp430` HAL, so these examples drive the
[`msp430fr2355`](https://docs.rs/msp430fr2355) PAC directly. What the port provides is the executor
and an `embassy-time` driver; the rest is a handful of register writes in `src/board.rs`.

| Example       | What it does                                                          |
|---------------|-----------------------------------------------------------------------|
| `blinky`      | Two LEDs blinking at different rates from two tasks.                   |
| `button`      | Awaits a GPIO interrupt from S1 and toggles an LED.                    |
| `low_power`   | The same blink, with the executor sleeping in LPM3 between timer ticks.|

## Toolchain

`msp430-none-elf` is a tier 3 target, so you need:

- A nightly compiler with `rust-src` (`rust-toolchain.toml` pins this). `core` is built from source
  via `-Zbuild-std`, configured in `.cargo/config.toml`.
- [TI's `msp430-gcc`](https://www.ti.com/tool/MSP430-GCC-OPENSOURCE), for `msp430-elf-gcc`. It is
  the linker, and it supplies `libgcc` — MSP430 has no hardware divider, so the 64-bit arithmetic
  in `embassy-time` lowers to `__mspabi_*` calls that live there.

  `rust-lld` is not a substitute: it cannot relocate `R_MSP430_SYM_DIFF`, which `msp430-rt`'s
  prebuilt startup object uses.

To flash and debug, [`mspdebug`](https://dlbeer.co.nz/mspdebug/) talks to the LaunchPad's on-board
eZ-FET; `.cargo/config.toml` wires it up as the cargo runner.

```sh
cargo build --release --bin blinky
cargo run --release --bin blinky      # needs mspdebug
```

## How the port fits together

### Sleeping

An MSP430 interrupt does not, by itself, bring the CPU out of a low-power mode: the status register
is stacked on entry and restored by `reti`, so a core that slept with `CPUOFF` set goes straight
back to sleep when the handler returns. Waking up means clearing the LPM bits in the *stacked* SR.

`embassy_executor::msp430_interrupt!` emits each vector as a naked trampoline that does exactly
that before jumping into the handler body. **Every interrupt that wakes a task must be declared
with it** rather than with `#[msp430_rt::interrupt]`, or its wakeup is lost until something else
happens to wake the CPU.

The executor itself sleeps with a single `bis #(LPM | GIE), SR`, which commits both bits at once so
a wakeup can't slip in between enabling interrupts and going to sleep.

### Which low-power mode

`embassy_executor::set_low_power_mode` picks the mode, at any point, including from a task. The
default is LPM0. Anything down to LPM3 keeps ACLK running, which is what the time driver needs; LPM4
stops it, so only a GPIO edge or a reset gets you back.

### Time

`src/time_driver.rs` drives `embassy-time` from Timer_B0 in continuous mode off ACLK, sourced from
the internal 32768 Hz REFO — no crystal required, and it survives LPM3. TB0's 16-bit counter is
extended to the 64 bits `embassy-time` wants by counting half-overflows in software; CCR0 serves as
the alarm. `embassy-time`'s tick rate is therefore fixed at 32768 Hz.

## Pin map

| Function | Pin  |
|----------|------|
| LED1     | P1.0 |
| LED2     | P6.6 |
| S1       | P4.1 |
