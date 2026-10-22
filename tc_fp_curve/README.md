# tc_fp_curve

[![crates.io](https://img.shields.io/crates/v/tc_fp_curve.svg)](https://crates.io/crates/tc_fp_curve)
[![docs.rs](https://docs.rs/tc_fp_curve/badge.svg)](https://docs.rs/tc_fp_curve)
[![CI](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Prime-field short-Weierstrass elliptic curves `y² = x³ + ax + b` for
cryptography, after Bouncy Castle's `FpCurve`, on the Montgomery arithmetic of
`tc_modular`, with the field chosen at run time. `no_std` with `alloc`, Rust
1.85 or later.

## Types

- `FpField` — the prime field GF(q), its modulus chosen at run time.
- `FpFieldElement` — an element of it, in Montgomery form, wiped on drop.
- `FpCtCurve`, `FpCtPoint` — the constant-time set: complete formulas and a 4-bit fixed window.
- `FpVarCurve`, `FpVarPoint` — the variable-time set: affine, homogeneous or Jacobian coordinates, wNAF, and GLV on a curve given one.
- `FpCtPrecomputed`, `FpVarPrecomputed` — `BaseMultiplier`s that keep the curve's own table for a fixed base.
- `FpError` — a modulus, coefficient or coordinate the field does not take.
- `named_curves::FpNamedCurve` — named parameters: `SECP256R1` (P-256), `SECP256K1`, and the weak curves of `small-curves`.

## Features

- `small-curves` (off by default) — secp112r1, secp112r2 and secp128r2: weak, for interoperability only.

## Usage

```rust
use tc_bigint::PaddedBigUint;
use tc_fp_curve::named_curves::SECP256R1;

// P-256 on the constant-time curve, as the scalar is secret.
let field = SECP256R1.field();
let curve = SECP256R1.ct_curve(&field);
let g = SECP256R1.ct_generator(&curve);
let q = g.multiply(&PaddedBigUint::from(0x1234_5678u32));
let encoded = q.encode(true);
```

The traits these types implement are in `tc_ec_core`; benchmarks of the two
sets are in [BENCHES.md](BENCHES.md).

## Security

The Ct set is constant time in the points and the scalar, apart from the
`bool` and `Option` answers. Its complete formulas, by Renes, Costello and
Batina, hold on groups of odd order, which every prime-order curve has. The
Var set is variable time, only for public values. Decoding an encoding is
variable time, as an encoding is public. No `unsafe`. This is a learning port
with no independent security audit.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
