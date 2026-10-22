# Changelog

All notable changes to `tc_fp_curve` are documented in this file.

## 0.1.0 - 2026-10-22

Initial release.

### Added

- `FpField` and `FpFieldElement`, a prime field chosen at run time on the
  Montgomery forms of `tc_modular`, with square roots.
- The constant-time set `FpCtCurve` and `FpCtPoint`: the complete formulas
  of Renes, Costello and Batina in homogeneous coordinates, and a 4-bit fixed
  window.
- The variable-time set `FpVarCurve` and `FpVarPoint`: affine, homogeneous
  and Jacobian coordinates, wNAF, and Bouncy Castle's GLV split on a curve
  given one by `with_glv`.
- `FpCtPrecomputed` and `FpVarPrecomputed`, `BaseMultiplier`s that keep the
  curve's own table.
- SEC 1 point encoding and decoding, and `FpError`.
- Named P-256 and secp256k1, with GLV on secp256k1, and, behind the
  default-off `small-curves` feature, secp112r1, secp112r2 and secp128r2.
- Unit and integration tests, a test that every public function and trait
  implementation documents whether it is constant or variable time, a doctest, and Criterion benchmarks of the two sets.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- `no_std`, but needs an allocator.
- Depends on `tc_ec_core` 0.1, `tc_bigint` 0.1 and `tc_modular` 0.1 with
  `alloc`, `tc_constant_time` 0.1 and `tc_zeroize` 0.1.
- The Ct set is constant time apart from its `bool` and `Option` answers;
  the Var set is variable time, only for public values.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
