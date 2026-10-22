# tc_ec_core

[![crates.io](https://img.shields.io/crates/v/tc_ec_core.svg)](https://crates.io/crates/tc_ec_core)
[![docs.rs](https://docs.rs/tc_ec_core/badge.svg)](https://docs.rs/tc_ec_core)
[![CI](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

The traits and algorithms shared by elliptic curves over prime and binary
fields, after Bouncy Castle's `Org.BouncyCastle.Math.EC`: what a curve crate
implements, and the scalar multiplications generic over it. `no_std` with
`alloc`, Rust 1.85 or later.

## Traits

- `Curve` — a curve: its coefficients, order, cofactor, points and multiplication.
- `Point` — a point: the group operations, validation and normalization.
- `FieldElement` — an element of a prime or binary field.
- `PrimeFieldElement` — a prime-field element, to and from an unsigned integer.
- `BinaryFieldElement` — a binary-field element, with its trace and half-trace.
- `Scalar` — bit access to a scalar, for recoding.
- `BaseMultiplier` — multiplication of a base fixed at construction, its table kept.

## Types

- `CoordinateSystem` — the coordinates a curve keeps its points in.
- `PointDecodeError`, `PointEncodeError` — why a SEC 1 point encoding failed.
- `wnaf::WNafTable` — the odd multiples of a point for wNAF multiplication.
- `glv::GlvTable`, `glv::ScalarSplit` — the table and the scalar split of GLV.
- `tnaf::WTauNafTable` — the table of the window τ-adic NAF on Koblitz curves.
- `multiplier::PerCall` — a base multiplied by the curve's own method each time.
- `multiplier::CombCt`, `multiplier::CombVar` — Bouncy Castle's fixed-point comb.
- `algo::AlgorithmError` — why point import, validation or normalization failed.

## Features

None. The crate needs an allocator, for its tables and digit strings.

## Usage

```rust
use tc_ec_core::Curve;
use tc_ec_core::multiplier::{BaseMultiplier, CombVar};

// Many public multiples of one base, through a comb built once.
fn multiples<C: Curve>(g: C::Point, scalars: &[C::Scalar]) -> Vec<C::Point> {
    let comb = CombVar::<C>::new(g);
    scalars.iter().map(|k| comb.multiply(k)).collect()
}
```

The curves come from `tc_fp_curve`, `tc_f2m_curve`, `tc_fp_custom` and
`tc_f2m_custom`.

## Security

Every item states whether it is constant or variable time. The wNAF, GLV and
τ-adic NAF multiplications and `CombVar` are variable time, only for public
scalars; constant-time multiplication comes from the curves' own Ct types and
from `CombCt`, which reads its whole table for every lookup. No `unsafe`. This
is a learning port with no independent security audit.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
