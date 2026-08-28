# msp430f149

Peripheral access API for MSP430F149 microcontrollers.

This crate is **generated**, and vendored here only because there is no `msp430f149` on crates.io.
It is not part of Embassy and nothing else in this repository depends on it; it belongs published on
its own, ideally alongside the other MSP430 PACs. Do not hand-edit it — regenerate instead.

## Regenerating

[`msp430_svd`](https://github.com/pftbest/msp430_svd) converts TI's DSLite device description into
an SVD, and [`svd2rust`](https://github.com/rust-embedded/svd2rust) turns that into this crate.

```sh
git clone https://github.com/pftbest/msp430_svd
cd msp430_svd && cargo run --release -- msp430f149

cargo install svd2rust --version 0.37.1 form
svd2rust -g -i msp430f149.svd --target msp430
form -i lib.rs -o src/ && rm lib.rs
find src -name '*.rs' | xargs rustfmt --edition 2021
```

The `Cargo.toml` here is written by hand: `svd2rust` does not emit one, and the `vcell` dependency
its generic code needs is easy to miss.

TI's DSLite files name few of their enumerated values, so many fields come out as `Mc0`, `Mc1` and
so on rather than anything meaningful. `msp430_svd` can apply
[`svdtools`](https://github.com/stm32-rs/svdtools) patches from its `overrides/devices` directory to
fix that up; there is no patch file for this device, so this crate is generated from the unpatched
SVD.
