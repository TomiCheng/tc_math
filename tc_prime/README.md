# tc_prime

[![crates.io](https://img.shields.io/crates/v/tc_prime.svg)](https://crates.io/crates/tc_prime)
[![docs.rs](https://docs.rs/tc_prime/badge.svg)](https://docs.rs/tc_prime)
[![CI](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_math/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Primality testing and prime generation for cryptography over the integers of
[`tc_bigint`](https://crates.io/crates/tc_bigint), after Bouncy Castle's
`Primes`: trial division, Miller-Rabin, the enhanced test of FIPS 186-4, and
Shawe-Taylor provable primes. `no_std`, no `unsafe`, Rust 1.85 or later.

## Types

- `MrOutput<T>` — what the enhanced Miller-Rabin test found: probably prime, or composite with or without a factor.
- `StOutput<T>` (`shawe-taylor`) — a provable prime, with the seed and counter the routine ends at.
- `StError` (`shawe-taylor`) — why the Shawe-Taylor routine gave no prime.

## Traits

- `Primality` — trial division, Miller-Rabin, the enhanced test and random probable primes, on `FixedBigUint<N>`, `PaddedBigUint` and `BigUint`.
- `ShaweTaylor` (`shawe-taylor`) — provable primes by FIPS 186-4 C.6 from a `tc_digest` digest and a seed.

## Features

- `alloc` (off by default) — the traits on `PaddedBigUint` and `BigUint`.
- `shawe-taylor` (off by default) — `ShaweTaylor`, which adds `tc_digest` and turns on `alloc`.

## Usage

```rust
use tc_bigint::{FixedBigUint, Word};
use tc_prime::Primality;

type U256 = FixedBigUint<{ 256 / Word::BITS as usize }>;

let p = U256::from(i128::MAX as u128); // 2^127 - 1, a prime
assert!(!p.has_small_factor());
assert!(p.is_probable_prime_to_base(&U256::from(2u8)));
```

## Security

Every test and generator is variable time, as Bouncy Castle's are: only for
public values, or for candidates whose timing a caller accepts showing, as key
generation does. The random values come from the `rand_core` generator the
caller gives, which has to be cryptographically secure for key generation.
This is a learning port with no independent security audit.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
