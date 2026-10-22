# Changelog

All notable changes to `tc_f2m_curve` are documented in this file.

## 0.1.0 - Unreleased

Initial release.

### Added

- `F2mField` and `F2mFieldElement`, GF(2^m) chosen at run time on the
  polynomials of `tc_binpoly`, with trace, half-trace and square roots.
- The Ct set `F2mCtCurve` and `F2mCtPoint`, whose multiplication is a
  constant-time Montgomery ladder.
- The variable-time set `F2mVarCurve` and `F2mVarPoint`: affine,
  homogeneous and lambda-projective coordinates, wNAF, and the window τ-adic
  NAF on Koblitz curves.
- `F2mVarPrecomputed`, a `BaseMultiplier` that keeps the Var curve's own
  table.
- SEC 1 point encoding and decoding, the half-trace check of the subgroup
  for cofactors 2 and 4, and `F2mError`.
- The 18 SEC 2 binary curves; sect113r1, sect113r2, sect131r1 and sect131r2
  behind the default-off `small-curves` feature.
- Unit and integration tests, a test that every public function and trait
  implementation documents whether it is constant or variable time, and a doctest.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- `no_std`, but needs an allocator.
- Depends on `tc_ec_core` 0.1, `tc_bigint` 0.1 and `tc_binpoly` 0.1 with
  `alloc`, `tc_constant_time` 0.1 and `tc_zeroize` 0.1.
- Only the Ct points' multiplication is constant time; the other group
  operations and the Var set are variable time, only for public values. The
  field is constant time on x86 with PCLMULQDQ; without it the product
  indexes a table by operand bits, a cache-timing side channel.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
