//! The traits and algorithms shared by elliptic curves over prime and binary fields,
//! after Bouncy Castle's `Org.BouncyCastle.Math.EC`.
//!
//! [`Curve`], [`Point`] and [`FieldElement`], with [`PrimeFieldElement`] and
//! [`BinaryFieldElement`], are what a curve crate implements; [`Scalar`] gives the
//! algorithms bit access to scalars. The algorithms are generic over them:
//!
//! - [`wnaf`]: NAF and wNAF recoding, tables of odd multiples, and sums of multiples.
//! - [`glv`]: the GLV split, on curves with an efficient endomorphism.
//! - [`tnaf`]: the window τ-adic NAF, on Koblitz curves.
//! - [`multiplier`]: fixed-base multipliers that keep a base's table, the comb among them.
//! - [`algo`]: point import, validation and batch normalization.
//!
//! Every item states whether it is constant time or variable time. The multiplications of
//! [`wnaf`], [`glv`] and [`tnaf`] are variable time, only for public scalars; constant-time
//! multiplication comes from the curves themselves and from [`multiplier::CombCt`].
//!
//! The crate needs an allocator, as its tables and digit strings live in vectors.

#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod algo;
mod coordinate_system;
mod errors;
pub mod glv;
pub mod multiplier;
pub mod tnaf;
mod traits;
pub mod wnaf;

pub use coordinate_system::CoordinateSystem;

pub use traits::BinaryFieldElement;
pub use traits::Curve;
pub use traits::FieldElement;
pub use traits::Point;
pub use traits::PrimeFieldElement;
pub use traits::Scalar;

pub use errors::PointDecodeError;
pub use errors::PointEncodeError;
