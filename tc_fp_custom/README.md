# tc_fp_custom

[![crates.io](https://img.shields.io/crates/v/tc_fp_custom.svg)](https://crates.io/crates/tc_fp_custom)
[![docs.rs](https://docs.rs/tc_fp_custom/badge.svg)](https://docs.rs/tc_fp_custom)
[![CI](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

The SEC prime curves and SM2 for cryptography with each field fixed at compile
time, after Bouncy Castle's `SecP*` and `SM2P256V1` classes: Solinas and
Mersenne reductions on fixed limb arrays, safegcd inversion, and the curves'
own square-root chains. About three times faster than the generic
`tc_fp_curve` on the same curve. `no_std` with `alloc`, Rust 1.85 or later.

## Types

- `SecPCtCurve`, `SecPCtPoint` — the constant-time set: complete formulas and a 4-bit fixed window.
- `SecPVarCurve`, `SecPVarPoint` — the variable-time set: Jacobian coordinates, wNAF, and GLV on the k1 curves.
- `SecPCtPrecomputed`, `SecPVarPrecomputed` — `BaseMultiplier`s that keep the curve's own table for a fixed base.
- `SecP128R1FieldElement` to `SecP521R1FieldElement`, `SM2P256V1FieldElement` — the fields, each its own type, wiped on drop.
- `SecPError` — a coordinate or encoding the curve does not take.
- `named_curves::SecPNamedCurve` — secp192k1 to secp521r1 and sm2p256v1, and the weak curves of `small-curves`.

## Features

- `small-curves` (off by default) — secp128r1, secp160k1, secp160r1 and secp160r2: weak, for interoperability only.

## Usage

```rust
use tc_bigint::PaddedBigUint;
use tc_fp_custom::named_curves::SECP256R1;

// P-256 on the constant-time curve, as the scalar is secret.
let curve = SECP256R1.ct_curve();
let g = SECP256R1.ct_generator(&curve);
let q = g.multiply(&PaddedBigUint::from(0x1234_5678u32));
let encoded = q.encode(true);
```

The traits these types implement are in `tc_ec_core`; benchmarks against
`tc_fp_curve` are in [BENCHES.md](BENCHES.md).

## Security

The field elements and the Ct set are constant time, apart from the `bool`
and `Option` answers: masks replace Bouncy Castle's branches on secret values.
The Var set is variable time, only for public values. Decoding an encoding is
variable time, as an encoding is public. No `unsafe`. This is a learning port
with no independent security audit.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
