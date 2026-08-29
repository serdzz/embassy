# msp430fr5043

Peripheral access API for MSP430FR5043 microcontrollers — the ultrasonic sensing part without the
segment LCD driver.

This crate is **generated**, and vendored here only because there is no `msp430fr5043` on crates.io.
It is not part of Embassy and nothing else in this repository depends on it; it belongs published on
its own. Do not hand-edit it — regenerate instead.

139 000 lines, and all but seven thousand of them are shared with `msp430fr5043-pac`'s sibling: the
two devices are register-identical apart from `LCD_C`, which is what this one does not have. On top
of the usual eUSCI, timers, ADC12_B, RTC_C, DMA, AES and CRC there are the four modules that make up
the ultrasonic front end — `SAPH_A`, `SDHS`, `UUPS` and `HSPLL` — and the `LEA` accelerator that
processes what they capture.

Both crates are vendored rather than one being derived from the other, because they are generated
rather than written: deduplicating them would mean hand-editing generated code, which is the thing
the regeneration instructions below exist to avoid.

## Regenerating

[`msp430_svd`](https://github.com/pftbest/msp430_svd) converts TI's DSLite device description into
an SVD, and [`svd2rust`](https://github.com/rust-embedded/svd2rust) turns that into this crate.

```sh
git clone https://github.com/pftbest/msp430_svd
cd msp430_svd && cargo run --release -- msp430fr5043

# Two defects in TI's description have to be repaired first; see below.
python3 ../msp430fr5043-pac/tools/fix-svd-enums.py msp430fr5043.svd msp430fr5043-fixed.svd

cargo install svd2rust --version 0.37.1 form
svd2rust -g -i msp430fr5043-fixed.svd --target msp430
form -i lib.rs -o src/ && rm lib.rs
mv generic.rs src/generic.rs        # `form` only splits lib.rs; -g writes this one separately
find src -name '*.rs' | xargs rustfmt --edition 2021
```

### Two repairs to TI's description

`tools/fix-svd-enums.py` makes exactly four changes, and prints each one:

```
RTCSSEL: renamed duplicate name LFXT to LFXT_1
RTCSSEL: renamed duplicate name RT1PS to RT1PS_3
RT1SSEL: renamed duplicate name RT0PS to RT0PS_3
TONE: ENABLE had value 0, which was taken; gave it the only free encoding 1
```

The first three are **duplicate names**. RTC_C's `RTCSSEL` calls both encoding 0 and encoding 1
"LFXT", and both 2 and 3 "RT1PS", because those encodings genuinely do select the same clock. Fine
as documentation, illegal as a Rust enum. Both encodings are kept, told apart by their value.

The fourth is a **duplicate value**, and it is a typo in TI's data: `SAPH_A`'s `PGCTL.TONE` gives
both "test tone disabled" and "test tone enabled" the value 0, while the field is one bit wide and
its own description says "PGCTL.TONE = 1". The script gives the duplicate the only free encoding.
Anything less clear-cut than that it drops rather than guesses at, and says so.

### `msp430_svd` needs a patch for this device family

The same patch the MSP430F2618 needed: its parser rejects the DMA block's 32-bit address registers,
which are declared alongside the 16-bit halves of the same registers at the same addresses. Skipping
registers wider than a word in `src/dslite_parser.rs` gets past it. See `msp430f2618-pac/README.md`
for why that is a local patch and not something to send upstream unchanged.

## What is missing from the generated API

The `Cargo.toml` here is written by hand: `svd2rust` does not emit one, and the `vcell` dependency
its generic code needs is easy to miss. Debug info is switched off for this crate in both profiles —
gigabytes of it, for 146 000 lines of register accessors nobody will step through.

## Note on the address space

The device has 64 kB of FRAM, running past 0xFFFF. Reaching the upper 24 kB needs the 20-bit
addressing of the MSP430X, which Rust's `msp430-none-elf` target does not have, so a linker script
for this chip stops at the vector table and about 40 kB is usable. That is a property of the target,
not of this crate — every register here is below 0x1000 and reachable.

The vector table starts at **0xFF92**, not the 0xFF90 in TI's own linker script: the SVD numbers
`SDHS` as vector 15 at 0xFFB0, which fixes the base two bytes higher.
