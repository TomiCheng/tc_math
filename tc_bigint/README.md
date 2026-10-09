# tc_bigint

[![crates.io](https://img.shields.io/crates/v/tc_bigint.svg)](https://crates.io/crates/tc_bigint)
[![docs.rs](https://docs.rs/tc_bigint/badge.svg)](https://docs.rs/tc_bigint)
[![CI](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Big integers for cryptography in three kinds, each unsigned and signed:
fixed-width on the stack, padded to a width chosen at run time, and arbitrary
precision. `no_std`, no `unsafe`, Rust 1.85 or later.

## Types

- `FixedBigUint<N>`, `FixedBigInt<N>` — `N` limbs on the stack, constant time, no allocator.
- `PaddedBigUint`, `PaddedBigInt` (`alloc`) — a width chosen when built, on the heap, constant time.
- `BigUint`, `BigInt` (`alloc`) — as many limbs as the value takes, variable time, for public values.
- `Limb`, `LimbArray<N>`, `Word`, `WideWord` — the storage, a 64-bit word on 64-bit targets and 32-bit otherwise.
- `ConversionError`, `ParseBigIntError` — a value that does not fit, and text that does not parse.

## Traits

- Operators — with an integer or a `u32` on the right, panicking on overflow in every build.
- `num-traits` — `Zero`, `One`, `Num`, checked, wrapping, saturating and overflowing arithmetic, `Pow`, `Euclid`, `Signed`, `Unsigned`, and `Bounded` on the fixed-width types.
- `BitOps`, `Gcd`, `ArrayEncoding` — bit access, greatest common divisors, and encoding to and from words or bytes.
- `RandomBits`, `RandomRange` (`rand_core`) — random values below a power of two or in a range.
- `ConstantTimeEq`, `ConstantTimeOrd`, `ConditionallySelectable`, `Zeroize` — from `tc_constant_time` and `tc_zeroize`.

## Features

- `alloc` (off by default) — the padded and big types.
- `rand_core` (off by default) — random integers from a `rand_core` generator.

## Usage

```rust
use tc_bigint::{FixedBigUint, Word};

type U256 = FixedBigUint<{ 256 / Word::BITS as usize }>;

let x = U256::from(u128::MAX);
let y = &x * &x + 7u32;
assert_eq!(y.div_rem(&x), (x, U256::from(7u8)));
```

Benchmarks of the three kinds are in [BENCHES.md](BENCHES.md).

## Security

Constant time covers the fixed-width and padded types; the big ones are for
public values only, and every method states which it is. The constant-time
types wipe the working values they give up; the values they hand back are the
caller's to wipe. This is a learning port with no independent security audit.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
