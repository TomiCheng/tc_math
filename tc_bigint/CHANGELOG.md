# Changelog

All notable changes to `tc_bigint` are documented in this file.

## 0.1.0 - 2026-10-09

Initial release.

### Added

- `FixedBigUint<N>` and `FixedBigInt<N>`: unsigned and two's complement
  integers of `N` limbs on the stack, constant time, with no allocator.
- With the default-off `alloc` feature, `PaddedBigUint` and `PaddedBigInt`,
  whose width is chosen when they are built and which stay constant time,
  extending the narrower operand of a binary operation to the wider width;
  and `BigUint` and `BigInt`, which keep as many limbs as their value takes
  and are variable time, for public values, with Karatsuba multiplication
  and Knuth's long division.
- `Limb`, `LimbArray<N>`, `Word` and `WideWord`: the storage, a 64-bit word
  on 64-bit targets and 32-bit otherwise.
- The arithmetic, bitwise, shift and negation operators by value and by
  reference, with an integer or a `u32` on the right, panicking on overflow in
  every build; `div_rem`, `gcd` and the bit operations as inherent methods.
- The `num-traits` contracts: `Zero`, `One`, `Num`, `FromPrimitive`,
  `ToPrimitive`, checked, wrapping, saturating and overflowing arithmetic,
  `Pow`, `Euclid`, `Signed` and `Unsigned`, and `Bounded` on the fixed-width
  types.
- `BitOps`, `Gcd`, and `ArrayEncoding` to and from little- or big-endian
  `u8`, `u32` or `u64` units, with `ConversionError`.
- Conversions between the three kinds and from and to the primitive
  integers, parsing in any radix from 2 to 36 with `ParseBigIntError`, and
  `Display`, `Binary`, `Octal`, `LowerHex` and `UpperHex`. `Debug` on `Limb`
  never prints the word.
- `ConstantTimeEq`, `ConstantTimeOrd` and `ConditionallySelectable` from
  `tc_constant_time`, and `Zeroize` from `tc_zeroize`; the constant-time
  types wipe the working values an operation gives up.
- With the default-off `rand_core` feature, `RandomBits` and `RandomRange`,
  which draw uniform values below a power of two or in a range from a
  `rand_core` generator.
- Unit tests against the primitive integers, a test that the operators
  allocate only where they must, a test that every public function and trait
  implementation documents whether it is constant or variable time, doctests,
  and Criterion benchmarks of the three kinds at 256 and 2048 bits.

### Compatibility

- Requires Rust 1.85 or later and uses Rust edition 2024.
- Depends on `num-traits` 0.2 without default features, `tc_constant_time`
  0.1 and `tc_zeroize` 0.1, and with `rand_core` on `rand_core` 0.10.
  Contains no `unsafe` code.
- The fixed-width and padded types run in constant time in their values;
  their widths, shift amounts and exponents are public. The big types are
  variable time and only for public values.
- Licensed under MIT OR Apache-2.0; both license texts are included in the
  published package.
