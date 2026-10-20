//! Binary-polynomial arithmetic over GF(2^n) for cryptography, after Bouncy Castle's
//! `BinPoly`: products, squares and inverses modulo a trinomial or a pentanomial, as the
//! binary elliptic curves of SEC 2 and X9.62 use them.
//!
//! A [`BinField`] names the field by its modulus and picks, once, the multiplication and
//! squaring backends this CPU supports. A polynomial reduced in it is a [`FixedBinPoly<N>`],
//! `N` limbs on the stack, or, with `alloc`, a `PaddedBinPoly` on the heap. Both add,
//! multiply, square, invert by Itoh–Tsujii, read and write little- and big-endian bytes,
//! and implement `ConstantTimeEq`, `ConditionallySelectable` and `Zeroize`.
//!
//! # Timing
//!
//! Every operation is constant time in the coefficients on x86 with PCLMULQDQ, which every
//! x86 processor of the last decade has. Elsewhere the product indexes a 16-entry table by
//! bits of the operands, as Bouncy Castle's fallback does: a cache-timing side channel.
//! Squaring is constant time everywhere, by PDEP where the CPU has BMI2. The field is
//! public. Working values are wiped by volatile writes; the values returned are the
//! caller's to wipe, and no type wipes on drop.
//!
//! # Features
//!
//! - `alloc` (off by default) adds `PaddedBinPoly`, `to_le` and `to_be`, and fields past
//!   degree 4096, up to 2^20, whose working buffers outgrow the stack.
//!
//! # Example
//!
//! ```
//! use tc_binpoly::{BinField, FixedBinPoly};
//!
//! // GF(2^163), the field of SEC 2's sect163 curves.
//! let field = BinField::pentanomial(163, 3, 6, 7)?;
//! let x = FixedBinPoly::<3>::from_limbs(field, [0x1234_5678, 0, 1 << 34])?;
//! let product = x.multiply(&x.invert());
//! assert_eq!(product, FixedBinPoly::one(field)?);
//! # Ok::<(), tc_binpoly::BinPolyError>(())
//! ```

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

mod bin_field;
mod fixed_bin_poly;
#[cfg(feature = "alloc")]
mod padded_bin_poly;

mod encoding;
mod errors;
mod invert;
mod multiply;
mod reduce;
mod square;

pub use bin_field::BinField;
pub use errors::BinPolyError;
pub use fixed_bin_poly::FixedBinPoly;
#[cfg(feature = "alloc")]
pub use padded_bin_poly::PaddedBinPoly;
