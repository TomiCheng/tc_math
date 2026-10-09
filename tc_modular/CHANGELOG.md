# Changelog

All notable changes to `tc_modular` are documented in this file.

## 0.1.0 - Unreleased

Initial release.

### Added

- `NonZero<T>` and `Odd<T>`, which check a value once to be non-zero or odd
  and give it back through `Deref`, `AsRef` and `into_inner`.
- The `ModAdd`, `ModSub`, `ModMul`, `ModPow` and `ModInverse` traits, taking
  operands of any size and a `NonZero` modulus, implemented for
  `FixedBigUint<N>` of `tc_bigint` and, with the default-off `alloc` feature,
  for `PaddedBigUint` and `BigUint`. `ModPow` goes through Montgomery form
  for an odd modulus and squares and multiplies on the residues for an even
  one; `ModInverse` uses safegcd for an odd modulus and the binary extended
  greatest common divisor for an even one.
- `FixedMontyParams<N>` and, with `alloc`, `PaddedMontyParams` and
  `BigMontyParams`: `R mod m`, `R² mod m` and `-m⁻¹ mod 2^Word::BITS` for an
  odd modulus, worked out once.
- `FixedMontyForm<N>` and, with `alloc`, `PaddedMontyForm` and
  `BigMontyForm`: a value kept as `x · R mod m`, with addition, subtraction,
  multiplication and negation by value and by reference, `square`, `double`,
  `pow` in constant time and `pow_vartime` for a public exponent, `invert`
  and `retrieve`. The fixed and padded forms implement `ConstantTimeEq` and
  `ConditionallySelectable`, and every form and parameter set implements
  `Zeroize`.
- Unit tests against arithmetic on the primitive integers, a test that every
  public function and trait implementation documents whether it is constant
  or variable time, doctests, and Criterion benchmarks of the three kinds at
  256 and 2048 bits.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- Depends on `tc_bigint` 0.1, `num-traits` 0.2 without default features,
  `tc_constant_time` 0.1 and `tc_zeroize` 0.1. Contains no `unsafe` code.
- Over the fixed-width and padded integers every operation is constant time
  in its values, apart from the parity of the modulus, which picks the path of
  `ModPow` and `ModInverse`, and whether an inverse exists. Over `BigUint`,
  going in and out of a form is variable time and only for public values.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
