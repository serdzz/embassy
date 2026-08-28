# Embassy on the MSP430FR2355 LaunchPad

Examples for the [MSP-EXP430FR2355](https://www.ti.com/tool/MSP-EXP430FR2355) LaunchPad, built on
`embassy-msp430` and `embassy-executor`'s `platform-msp430`.

| Example     | What it does                                                           |
|-------------|------------------------------------------------------------------------|
| `blinky`    | Two LEDs blinking at different rates from two tasks.                    |
| `button`    | Awaits a GPIO interrupt from S1 and toggles an LED.                     |
| `low_power` | Runs MCLK at 8 MHz and sleeps in LPM3 between timer events.             |
| `uart`      | Echoes characters over the backchannel UART while an LED blinks.        |
| `spi_loopback` | SPI self-test: jumper P1.2 to P1.3 and watch the blink rate.         |
| `i2c_scan`  | Scans the I2C bus and reports what answers, over the UART.              |
| `adc`       | Reads A1 against the internal 1.5 V reference and prints millivolts.    |
| `pwm`       | Two LEDs breathing in antiphase from one timer (needs external LEDs).   |
| `rtc`       | A heartbeat from the RTC off the VLO, with the executor in LPM3.        |

## Toolchain

`msp430-none-elf` is a tier 3 target, so you need:

- A nightly compiler with `rust-src` (`rust-toolchain.toml` pins this). `core` is built from source
  via `-Zbuild-std`, configured in `.cargo/config.toml`.
- [TI's `msp430-gcc`](https://www.ti.com/tool/MSP430-GCC-OPENSOURCE), for `msp430-elf-gcc`. It is
  the linker, and it supplies `libgcc` — MSP430 has neither a divider nor a multiplier, so
  arithmetic wider than a shift lowers to `__mspabi_*` calls that live there.

  `rust-lld` is not a substitute: it cannot relocate `R_MSP430_SYM_DIFF`, which `msp430-rt`'s
  prebuilt startup object uses.

To flash and debug, [`mspdebug`](https://dlbeer.co.nz/mspdebug/) talks to the LaunchPad's on-board
eZ-FET; `.cargo/config.toml` wires it up as the cargo runner.

```sh
cargo build --release --bin blinky
cargo run --release --bin blinky      # needs mspdebug
```

## Waking up from sleep

An MSP430 interrupt does not, by itself, bring the CPU out of a low-power mode: the status register
is stacked on entry and restored by `reti`, so a core that slept with `CPUOFF` set goes straight
back to sleep when the handler returns. Waking up means clearing the LPM bits in the *stacked* SR.

`embassy_executor::msp430_interrupt!` emits each vector as a naked trampoline that does exactly
that before jumping into the handler body. The handlers inside `embassy-msp430` already use it;
**any interrupt you add yourself that wakes a task must use it too**, rather than
`#[msp430_rt::interrupt]`, or its wakeup is lost until something else wakes the CPU.

`embassy_executor::set_low_power_mode` picks how deeply the executor sleeps, at any point,
including from a task. The default is LPM0. Anything down to LPM3 keeps ACLK running, which is what
the time driver needs; LPM4 stops it, so only a GPIO edge or a reset gets you back.

## Pin map

| Function | Pin  |
|----------|------|
| LED1     | P1.0 |
| LED2     | P6.6 |
| S1       | P4.1 |
