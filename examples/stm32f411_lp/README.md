# Low Power Examples for the STM32F411

Written for the WeAct "Black Pill" (STM32F411CEU6): LED on `PC13` (active low), KEY button on
`PA0` (shorts to GND when pressed).

These examples use the `low-power` feature of `embassy-stm32` together with the STM32-specific
executor (`embassy_stm32::executor::Executor`). Whenever no task is ready to run and the next
timer deadline is further out than `Config::min_stop_pause` (250 ms by default), the executor
parks the chip in STOP mode. The general purpose timer that drives `embassy-time` is paused and
the RTC wakeup timer takes over as the time source, so `Timer::after_*` keeps working across
STOP.

## Examples

* `stop` — blinks the `PC13` LED with a 300 ms period; the chip sits in STOP between toggles.
  It halts after 30 s so the debugger can take over again (relevant when
  `enable_debug_during_sleep` is turned off).
* `button_exti` — toggles the `PC13` LED on every KEY press on `PA0`. Nothing is pending between
  presses, so the chip stays in STOP and the EXTI line wakes it up. The LED state survives STOP,
  so this one can be checked without a debugger attached.

Run them with:

```
cargo run --release --bin stop
```

Note that RTT goes quiet while the chip is in STOP — the host cannot drain the buffer with the
core asleep. `stop` wakes every 300 ms and so keeps logging; `button_exti` stops logging once it
parks in STOP, until you press the button. For the same reason, flashing can fail with
`SwdApWait`/`SwdDpWait` when the chip is asleep: just retry, or hold BOOT0 and tap NRST to bring
the board up in the ROM bootloader first.

## Clocks

The F411 wakes from STOP running on the HSI, and `exit_stop` does not restore the clock tree on
F4, so these examples stay on the default configuration (HSI as SYSCLK, PLL off). If you enable
the PLL, re-configure RCC yourself after each wakeup.

The RTC needs a low-speed clock. The examples select the LSI (`LsConfig::default_lsi()`), which
is always available but drifts by a few percent. The Black Pill has a 32.768 kHz crystal fitted,
so `LsConfig::default_lse()` is the better choice if you care about timing accuracy across STOP.

## Measuring power

`Config::enable_debug_during_sleep` defaults to `true` so that RTT/debugging keeps working
during STOP, at a significant cost in current draw. Set it to `false` before taking any
measurement. On the Black Pill there is no IDD jumper, so you have to break the 3V3 supply
yourself to insert an ammeter — and note that the on-board LDO and the USB-C CC resistors draw
current of their own regardless of what the MCU is doing.

## Checklist before running examples

You might need to adjust `.cargo/config.toml`, `Cargo.toml` and possibly update pin numbers or
peripherals to match the specific MCU or board you are using.

* [ ] Update `.cargo/config.toml` with the correct probe-rs command for your MCU (use
  `probe-rs chip list` to find your chip).
* [ ] Update `Cargo.toml` to have the correct `embassy-stm32` chip feature, e.g. `stm32f411re`
  for a Nucleo-F411RE. Look in the `Cargo.toml` file of the `embassy-stm32` project to find the
  correct feature flag for your chip.
* [ ] If your board has a special clock or power configuration, make sure that it is set up
  appropriately.
* [ ] If your board has a different pin mapping, update any pin numbers or peripherals in the
  given example code to match your schematic.

If you are unsure, please drop by the Embassy Matrix chat for support, and let us know:

* Which example you are trying to run
* Which chip and board you are using

Embassy Chat: https://matrix.to/#/#embassy-rs:matrix.org
