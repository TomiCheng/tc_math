# tc_f2m_custom

[![crates.io](https://img.shields.io/crates/v/tc_f2m_custom.svg)](https://crates.io/crates/tc_f2m_custom)
[![docs.rs](https://docs.rs/tc_f2m_custom/badge.svg)](https://docs.rs/tc_f2m_custom)
[![CI](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

The SEC binary curves for cryptography with each field fixed at compile time,
after Bouncy Castle's `SecT*` classes: written-out reductions on fixed limb
arrays, products by PCLMULQDQ and squares by BMI2 on x86, chosen at run time,
and Bouncy Castle's portable kernels elsewhere. About five times faster than
the generic `tc_f2m_curve` on the same curve. `no_std` with `alloc`, Rust 1.85
or later.

## Types

- `SecTCtCurve`, `SecTCtPoint` — the Ct set: affine points whose `multiply` is a constant-time Montgomery ladder.
- `SecTVarCurve`, `SecTVarPoint` — the variable-time set: lambda-projective coordinates, wNAF, and the τ-adic NAF on Koblitz curves.
- `SecTVarPrecomputed` — a `BaseMultiplier` that keeps the Var curve's own table for a fixed base.
- `SecT113FieldElement` to `SecT571FieldElement` — the fields, each its own type, wiped on drop.
- `SecTError` — a coordinate or encoding the curve does not take.
- `named_curves::SecTNamedCurve` — the 18 SEC 2 binary curves, four of them behind `small-curves`.

## Features

- `small-curves` (off by default) — sect113r1, sect113r2, sect131r1 and sect131r2: weak, for interoperability only.

## Usage

```rust
use tc_bigint::PaddedBigUint;
use tc_f2m_custom::named_curves::SECT283K1;

// sect283k1 on the constant-time curve, as the scalar is secret.
let curve = SECT283K1.ct_curve();
let g = SECT283K1.ct_generator(&curve);
let q = g.multiply(&PaddedBigUint::from(0x1234_5678u32));
let encoded = q.encode(true);
```

The traits these types implement are in `tc_ec_core`; benchmarks against
`tc_f2m_curve` are in [BENCHES.md](BENCHES.md).

## Security

On the Ct curve only a point's `multiply`, the Montgomery ladder, is constant
time in the scalar; the other group operations are Bouncy Castle's affine
formulas, variable time and only for public points. The Var set is variable
time, only for public values. On x86 with PCLMULQDQ the field arithmetic is
constant time; without it the product indexes a table by bits of the
operands, as Bouncy Castle's portable kernel does: a cache-timing side
channel. `unsafe` is confined to the x86 intrinsics, each behind a
`tc_runtime` proof that the CPU has the instruction. This is a learning port
with no independent security audit.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
