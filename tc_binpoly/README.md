# tc_binpoly

[![crates.io](https://img.shields.io/crates/v/tc_binpoly.svg)](https://crates.io/crates/tc_binpoly)
[![docs.rs](https://docs.rs/tc_binpoly/badge.svg)](https://docs.rs/tc_binpoly)
[![CI](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Binary-polynomial arithmetic over GF(2^n) for cryptography, after Bouncy
Castle's `BinPoly`: products, squares and inverses modulo a trinomial or a
pentanomial, as binary elliptic curves use them. `no_std`, Rust 1.85 or later.

## Types

- `BinField` — the field, by its trinomial or pentanomial modulus, with the backends picked once for this CPU.
- `FixedBinPoly<N>` — a reduced polynomial in `N` limbs on the stack.
- `PaddedBinPoly` (`alloc`) — a reduced polynomial on the heap, its length set by the field.
- `BinPolyError` — a modulus, length or value the field does not take.

## Features

- `alloc` (off by default) — `PaddedBinPoly`, `to_le` and `to_be`, and degrees past 4096, up to 2^20.

## Usage

```rust
use tc_binpoly::{BinField, FixedBinPoly};

// GF(2^163), the field of SEC 2's sect163 curves.
let field = BinField::pentanomial(163, 3, 6, 7)?;
let x = FixedBinPoly::<3>::from_limbs(field, [0x1234_5678, 0, 1 << 34])?;
assert_eq!(x.multiply(&x.invert()), FixedBinPoly::one(field)?);
# Ok::<(), tc_binpoly::BinPolyError>(())
```

Benchmarks of the backends are in [BENCHES.md](BENCHES.md).

## Security

On x86 with PCLMULQDQ every operation is constant time in the coefficients.
Without it, the product indexes a table by bits of the operands, as Bouncy
Castle's fallback does: a cache-timing side channel. Squaring is constant time
everywhere. `unsafe` is confined to the x86 intrinsics, each behind a proof
that the CPU has the instruction. Working values are wiped; the values handed
back are the caller's to wipe. This is a learning port with no independent
security audit.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
