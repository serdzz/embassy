# msp430fr4133

Peripheral access API for MSP430FR4133 microcontrollers — the LCD-driving FR4xx value line part on
the MSP-EXP430FR4133 LaunchPad.

This crate is **generated**, and vendored here because the `msp430fr4133` crate that does exist on
crates.io is from 2020 and built against `msp430` 0.2 / `msp430-rt` 0.2, with no `critical-section`
support — nothing in this repository can link against that. This one follows the same
`msp430` 0.4 / `msp430-rt` 0.4 conventions as the `msp430fr2355` 0.6 crate the rest of
`embassy-msp430` uses. It is not part of Embassy and nothing else in this repository depends on it;
it belongs published on its own (under a different name). Do not hand-edit it — regenerate instead.

The device is the FR2xx/FR4xx generation: eUSCI_A0 (UART/IrDA/SPI), eUSCI_B0 (SPI/I2C), two
Timer_A3 instances (TA0, TA1 — no Timer_B on this part), the 16-bit RTC counter, a 10-bit ADC, the
LCD_E segment controller, CRC16, capacitive touch I/O and ports P1 through P8.

## Regenerating

[`msp430_svd`](https://github.com/pftbest/msp430_svd) converts TI's DSLite device description into
an SVD, and [`svd2rust`](https://github.com/rust-embedded/svd2rust) turns that into this crate.

```sh
git clone https://github.com/pftbest/msp430_svd
cd msp430_svd && cargo run --release -- msp430fr4133

# The enum-repair pass the FR5043/FR6043 needed; on this device it finds nothing to fix,
# run it anyway so a future TI description defect does not slip through silently.
python3 ../msp430fr5043-pac/tools/fix-svd-enums.py msp430fr4133.svd msp430fr4133-fixed.svd

cargo install svd2rust --version 0.37.1 form
svd2rust -g -i msp430fr4133-fixed.svd --target msp430
form -i lib.rs -o src/ && rm lib.rs
mv generic.rs src/generic.rs        # `form` only splits lib.rs; -g writes this one separately
find src -name '*.rs' | xargs rustfmt --edition 2021
```

Unlike the FR5043/FR6043, no patch to `msp430_svd` is needed: the FR4133 has no DMA block, which is
where the 32-bit register parse failure on those parts came from.

The `Cargo.toml` here is written by hand: `svd2rust` does not emit one, and the `vcell` dependency
its generic code needs is easy to miss.

## Note on the address space

15 kB of FRAM at 0xC400–0xFF7F, 2 kB of SRAM at 0x2000–0x27FF, everything below 0xFFFF — no 20-bit
addressing games on this part, the whole device is reachable from Rust's `msp430-none-elf` target.
The vector table runs 0xFF88–0xFFFF (59 slots plus reset); 0xFF80–0xFF87 holds the JTAG/BSL
signatures and must be left alone.
