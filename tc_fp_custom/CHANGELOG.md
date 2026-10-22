# Changelog

All notable changes to `tc_fp_custom` are documented in this file.

## 0.1.0 - Unreleased

Initial release.

### Added

- The fields of the SEC prime curves and SM2, `SecP128R1FieldElement` to
  `SecP521R1FieldElement` and `SM2P256V1FieldElement`, after Bouncy Castle's
  `SecP*Field` classes: Solinas and Mersenne reductions, safegcd inversion
  and the curves' square-root chains, with P-384 and P-521 multiplying on
  64-bit limbs.
- The constant-time set `SecPCtCurve` and `SecPCtPoint` and the
  variable-time set `SecPVarCurve` and `SecPVarPoint`, with GLV on the k1
  curves.
- `SecPCtPrecomputed` and `SecPVarPrecomputed`, `BaseMultiplier`s that
  keep the curve's own table.
- SEC 1 point encoding and decoding, and `SecPError`.
- Named secp192k1, secp192r1, secp224k1, secp224r1, secp256k1, secp256r1,
  secp384r1, secp521r1 and sm2p256v1; secp128r1, secp160k1, secp160r1 and
  secp160r2 behind the default-off `small-curves` feature.
- Tests against big-integer oracles and the generic curves, a test that
  every public function and trait implementation documents whether it is
  constant or variable time, a doctest, and Criterion benchmarks against
  `tc_fp_curve`.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- `no_std`, but needs an allocator.
- Depends on `tc_ec_core` 0.1, `tc_bigint` 0.1 with `alloc`,
  `tc_constant_time` 0.1 and `tc_zeroize` 0.1.
- The fields and the Ct set are constant time apart from their `bool` and
  `Option` answers; the Var set is variable time, only for public values.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
