# tc_modular

[![crates.io](https://img.shields.io/crates/v/tc_modular.svg)](https://crates.io/crates/tc_modular)
[![docs.rs](https://docs.rs/tc_modular/badge.svg)](https://docs.rs/tc_modular)
[![CI](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Modular arithmetic for cryptography over the integers of
[`tc_bigint`](https://crates.io/crates/tc_bigint): the `Mod*` operations, and
Montgomery parameters and forms for an odd modulus. `no_std`, no `unsafe`,
Rust 1.85 or later.

## Types

- `NonZero<T>`, `Odd<T>` — a value checked once to be non-zero, or odd, as a modulus must be.
- `FixedMontyParams<N>`, `PaddedMontyParams` (`alloc`), `BigMontyParams` (`alloc`) — what Montgomery arithmetic modulo an odd modulus works out once.
- `FixedMontyForm<N>`, `PaddedMontyForm` (`alloc`), `BigMontyForm` (`alloc`) — a value kept as `x · R mod m`, with the operators, `square`, `double`, `pow` and `invert`.

## Traits

- `ModAdd`, `ModSub`, `ModMul`, `ModPow`, `ModInverse` — on `FixedBigUint<N>`, `PaddedBigUint` and `BigUint`, over a `NonZero` modulus of any parity.

## Features

- `alloc` (off by default) — the padded and big types, and the traits on them.

## Usage

```rust
use tc_bigint::{FixedBigUint, Word};
use tc_modular::{FixedMontyForm, FixedMontyParams, Odd};

type U256 = FixedBigUint<{ 256 / Word::BITS as usize }>;

let p = U256::from(i128::MAX as u128); // 2^127 - 1, a prime
let params = FixedMontyParams::new(Odd::new(p.clone()).unwrap());
let three = FixedMontyForm::new(&U256::from(3u8), params);
assert_eq!(three.pow(&(&p - 1u32)).retrieve(), U256::from(1u8));
```

## Security

Over the fixed-width and padded integers everything is constant time, except
for two things that show: the parity of the modulus, which picks the path of
`ModPow` and `ModInverse`, and whether an inverse exists. Over `BigUint`,
going in and out of a form is variable time, for public values only. Working
values are wiped; the values handed back are the caller's to wipe. This is a
learning port with no independent security audit.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
