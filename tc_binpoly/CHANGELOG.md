# Changelog

All notable changes to `tc_binpoly` are documented in this file.

## 0.1.0 - Unreleased

Initial release.

### Added

- `BinField`, the field GF(2^n) by a trinomial or pentanomial modulus of
  degree 3 to 4096, or up to 2^20 with `alloc`, which picks its
  multiplication and squaring backends once for the CPU, as Bouncy Castle's
  `BinPolys` does.
- `FixedBinPoly<N>` and, with the default-off `alloc` feature,
  `PaddedBinPoly`: a reduced polynomial with addition, multiplication,
  `square`, `square_n` and `invert` by Itoh–Tsujii, little- and big-endian
  byte encodings, `ConstantTimeEq`, `ConditionallySelectable` and `Zeroize`.
- `BinPolyError` for a modulus, length or value the field does not take.
- On x86, products by PCLMULQDQ, with Bouncy Castle's kernels for one to ten
  limbs and Karatsuba above, and squares by BMI2's PDEP; elsewhere Bouncy
  Castle's scalar kernels.
- Unit tests against bit-by-bit schoolbook products, a test that every public
  function and trait implementation documents whether it is constant or
  variable time, a doctest, and Criterion benchmarks of the SEC 2 fields.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- Depends on `tc_constant_time` 0.1 and `tc_zeroize` 0.1, and on x86 on
  `tc_runtime` 0.1 for the CPU's features.
- Uses `unsafe` only for the x86 intrinsics, each called behind a
  `tc_runtime` proof that the CPU has the instruction.
- Constant time in the coefficients on x86 with PCLMULQDQ. Without it the
  product indexes a table by bits of the operands, a cache-timing side
  channel. The field is public.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
