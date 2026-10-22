# tc_f2m_curve

[![crates.io](https://img.shields.io/crates/v/tc_f2m_curve.svg)](https://crates.io/crates/tc_f2m_curve)
[![docs.rs](https://docs.rs/tc_f2m_curve/badge.svg)](https://docs.rs/tc_f2m_curve)
[![CI](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Binary-field short-Weierstrass elliptic curves `y² + xy = x³ + ax² + b` over
GF(2^m) for cryptography, after Bouncy Castle's `F2mCurve`, on the
polynomials of `tc_binpoly`, with the field chosen at run time. `no_std` with
`alloc`, Rust 1.85 or later.

## Types

- `F2mField` — GF(2^m), by its trinomial or pentanomial modulus.
- `F2mFieldElement` — an element of it, wiped on drop.
- `F2mCtCurve`, `F2mCtPoint` — the Ct set: affine points whose `multiply` is a constant-time Montgomery ladder.
- `F2mVarCurve`, `F2mVarPoint` — the variable-time set: affine, homogeneous or lambda-projective coordinates, wNAF, and the τ-adic NAF on Koblitz curves.
- `F2mVarPrecomputed` — a `BaseMultiplier` that keeps the Var curve's own table for a fixed base.
- `F2mError` — a modulus, coefficient or coordinate the field does not take.
- `named_curves::F2mNamedCurve` — the 18 SEC 2 binary curves, `SECT113R1` to `SECT571R1`, four of them behind `small-curves`.

## Features

- `small-curves` (off by default) — sect113r1, sect113r2, sect131r1 and sect131r2: weak, for interoperability only.

## Usage

```rust
use tc_bigint::PaddedBigUint;
use tc_f2m_curve::named_curves::SECT283K1;

// sect283k1 on the constant-time curve, as the scalar is secret.
let curve = SECT283K1.ct_curve();
let g = SECT283K1.ct_generator(&curve);
let q = g.multiply(&PaddedBigUint::from(0x1234_5678u32));
let encoded = q.encode(true);
```

The traits these types implement are in `tc_ec_core`.

## Security

On the Ct curve only a point's `multiply`, the Montgomery ladder, is constant
time in the scalar; the other group operations are Bouncy Castle's affine
formulas, variable time and only for public points. The Var set is variable
time, only for public values. The field arithmetic is `tc_binpoly`'s: constant
time on x86 with PCLMULQDQ, while without it the product indexes a table by
bits of the operands, a cache-timing side channel. No `unsafe`. This is a
learning port with no independent security audit.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
