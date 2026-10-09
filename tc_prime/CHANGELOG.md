# Changelog

All notable changes to `tc_prime` are documented in this file.

## 0.1.0 - 2026-10-09

Initial release.

### Added

- The `Primality` trait, after Bouncy Castle's `Primes`: trial division by
  the primes below 212, rounds of Miller-Rabin to random bases or to a given
  one, the enhanced Miller-Rabin test of FIPS 186-4 C.3.2, and random
  probable primes of an exact bit length. It is implemented for
  `FixedBigUint<N>` of `tc_bigint` and, with the default-off `alloc` feature,
  for `PaddedBigUint` and `BigUint`, and runs Miller-Rabin in Montgomery form
  through `tc_modular`.
- `MrOutput<T>`, the result of the enhanced test: probably prime, composite
  with a factor, or composite and not the power of a prime.
- With the default-off `shawe-taylor` feature, which turns on `alloc`, the
  `ShaweTaylor` trait: provable primes by the routine of FIPS 186-4 C.6 from
  a `tc_digest` digest and a seed, with `StOutput<T>` holding the prime, seed
  and counter, and `StError`.
- Unit tests of every integer type, including Shawe-Taylor primes over SHA-1
  and SHA-256 at fixed seeds, a test that every public function and trait
  implementation documents whether it is constant or variable time, and
  doctests.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- Depends on `tc_bigint` 0.1 with its `rand_core` feature, `tc_modular` 0.1,
  `num-traits` 0.2 and `rand_core` 0.10 without default features, and with
  `shawe-taylor` on `tc_digest` 0.1. Contains no `unsafe` code.
- Every test and generator is variable time, for public values or for
  candidates whose timing a caller accepts showing, as key generation does.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
