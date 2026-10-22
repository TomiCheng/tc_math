# Changelog

All notable changes to `tc_ec_core` are documented in this file.

## 0.1.0 - 2026-10-22

Initial release.

### Added

- The traits a curve crate implements, after Bouncy Castle's
  `Org.BouncyCastle.Math.EC`: `Curve`, `Point`, `FieldElement`,
  `PrimeFieldElement` and `BinaryFieldElement`, with `Scalar` for bit access
  to scalars and `CoordinateSystem`.
- `PointDecodeError` and `PointEncodeError` for SEC 1 point encodings.
- `wnaf`: NAF, wNAF, compact wNAF and joint sparse form recodings, the
  `WNafTable` of odd multiples, wNAF multiplication, and sums of multiples by
  Shamir's trick and interleaved wNAF.
- `glv`: Bouncy Castle's GLV split, `ScalarSplit`, and multiplication through
  a `GlvTable` of P and φ(P), with φ given by the caller.
- `tnaf`: the window τ-adic NAF on Koblitz curves, with its partial modular
  reduction, through a `WTauNafTable`, with τ given by the caller.
- `multiplier`: the `BaseMultiplier` trait for a base fixed at construction,
  `PerCall`, and Bouncy Castle's fixed-point comb as `CombCt` and `CombVar`.
- `algo`: point import, validation and batch normalization by Montgomery's
  trick.
- A test that every public function and trait implementation documents
  whether it is constant or variable time.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- `no_std`, but needs an allocator.
- Depends on `tc_bigint` 0.1 with `alloc`, and `tc_constant_time` 0.1.
- The wNAF, GLV and τ-adic NAF multiplications and `CombVar` are variable
  time, only for public scalars. `CombCt` is constant time in the scalar
  where the points' group operations are.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
