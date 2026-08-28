# msp430f2618

Peripheral access API for MSP430F2618 microcontrollers.

This crate is **generated**, and vendored here only because there is no `msp430f2618` on crates.io.
It is not part of Embassy and nothing else in this repository depends on it; it belongs published on
its own, ideally alongside the other MSP430 PACs. Do not hand-edit it — regenerate instead.

## Regenerating

[`msp430_svd`](https://github.com/pftbest/msp430_svd) converts TI's DSLite device description into
an SVD, and [`svd2rust`](https://github.com/rust-embedded/svd2rust) turns that into this crate.

```sh
git clone https://github.com/pftbest/msp430_svd
cd msp430_svd && cargo run --release -- msp430f2618

cargo install svd2rust --version 0.37.1 form
svd2rust -g -i msp430f2618.svd --target msp430
form -i lib.rs -o src/ && rm lib.rs
find src -name '*.rs' | xargs rustfmt --edition 2021
```

### `msp430_svd` needs a patch for this device

It panics on the F2618 as it stands. The DMA block declares its source and destination address
registers as 32 bits wide *and* declares the 16-bit halves of the same registers at the same
addresses, which the parser reads as overlapping registers and refuses.

The fix used here was to skip registers wider than a word in `src/dslite_parser.rs`:

```rust
if r.width > 2 {
    continue;
}
```

That is a local patch and **not something to send upstream** as it stands: it is right for a 16-bit
target, where a 32-bit register cannot be accessed in one instruction anyway and the halves are what
the code wants, but it would be wrong for a parser meant to serve MSP430X code too. Nothing in this
crate loses anything by it — the halves are all present.

## What is missing from the generated API

The `Cargo.toml` here is written by hand: `svd2rust` does not emit one, and the `vcell` dependency
its generic code needs is easy to miss.

TI's DSLite files name few of their enumerated values, so many fields come out as `Mc0`, `Mc1` and
so on rather than anything meaningful. `msp430_svd` can apply
[`svdtools`](https://github.com/stm32-rs/svdtools) patches from its `overrides/devices` directory to
fix that up; there is no patch file for this device, so this crate is generated from the unpatched
SVD.

## Note on the address space

The device has 116 kB of flash, running past 0xFFFF. Reaching the upper part needs the 20-bit
addressing of the MSP430X, which Rust's `msp430-none-elf` target does not have, so a linker script
for this chip stops at the vector table and about 50 kB is usable. That is a property of the target,
not of this crate — every register here is below 0x1000 and reachable.
