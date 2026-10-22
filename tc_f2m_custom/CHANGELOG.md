# Changelog

All notable changes to `tc_f2m_custom` are documented in this file.

## 0.1.0 - Unreleased

Initial release.

### Added

- The fields of the SEC binary curves, `SecT113FieldElement` to
  `SecT571FieldElement`, after Bouncy Castle's `SecT*Field` classes:
  written-out reductions, products by PCLMULQDQ and squares by BMI2 on x86,
  chosen at run time, and Bouncy Castle's portable kernels elsewhere.
- The Ct set `SecTCtCurve` and `SecTCtPoint`, whose multiplication is a
  constant-time Montgomery ladder, and the variable-time set `SecTVarCurve`
  and `SecTVarPoint`, with the window τ-adic NAF on Koblitz curves.
- `SecTVarPrecomputed`, a `BaseMultiplier` that keeps the Var curve's own
  table.
- SEC 1 point encoding and decoding, and `SecTError`.
- The 18 SEC 2 binary curves; sect113r1, sect113r2, sect131r1 and sect131r2
  behind the default-off `small-curves` feature.
- Tests against `tc_binpoly` and the generic curves, a test that every
  public function and trait implementation documents whether it is constant
  or variable time, a doctest, and Criterion benchmarks against
  `tc_f2m_curve`.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- `no_std`, but needs an allocator.
- Depends on `tc_ec_core` 0.1, `tc_bigint` 0.1 with `alloc`,
  `tc_constant_time` 0.1 and `tc_zeroize` 0.1, and on x86 on `tc_runtime`
  0.1 for the CPU's features.
- Uses `unsafe` only for the x86 intrinsics, each called behind a
  `tc_runtime` proof that the CPU has the instruction.
- Only the Ct points' multiplication is constant time; the other group
  operations and the Var set are variable time, only for public values. The
  field is constant time on x86 with PCLMULQDQ; without it the product
  indexes a table by operand bits, a cache-timing side channel.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
