//! Modular arithmetic for cryptography, over the integers of `tc_bigint`.
//!
//! It builds on that crate's public interface alone: values are read
//! through their limbs and built back from them, and the limb arithmetic it
//! needs is its own.
//!
//! The `Mod*` traits take an operand of any size and a modulus that is not
//! zero, as [`NonZero`] holds it. The Montgomery parameters and forms take
//! an odd modulus, as [`Odd`] holds it, and keep a value as `x · R mod m`, so
//! that a product takes one Montgomery multiplication rather than a
//! division.
//!
//! ```
//! use tc_bigint::{FixedBigUint, Word};
//! use tc_modular::{FixedMontyForm, FixedMontyParams, ModInverse, NonZero, Odd};
//!
//! type U256 = FixedBigUint<{ 256 / Word::BITS as usize }>;
//!
//! let p = U256::from(i128::MAX as u128); // 2^127 - 1, a prime
//! let params = FixedMontyParams::new(Odd::new(p.clone()).unwrap());
//! let three = FixedMontyForm::new(&U256::from(3u8), params);
//! assert_eq!(three.pow(&(&p - 1u32)).retrieve(), U256::from(1u8));
//! let inverse = U256::from(3u8).mod_inverse(&NonZero::new(p).unwrap());
//! assert_eq!(inverse, Some(U256::from(u128::MAX / 3)));
//! ```
//!
//! # Timing
//!
//! Over the fixed-width and padded integers, everything is constant time:
//! the running time follows the widths, which are public, and not the
//! values. The one thing a modulus shows is its parity, where it picks the
//! path, as for `ModPow` and `ModInverse`; whether an inverse exists shows
//! in the `Option` that holds it. Over the big integers, which are only for
//! public values, the way in and out is variable time. Every method says
//! which it is.
//!
//! # Wiping
//!
//! Over the fixed-width and padded integers, the working values an
//! operation makes and then gives up are wiped through volatile writes: the
//! residues of the operands, the steps of a power, the working values of an
//! inversion, the parameters a single `ModPow` builds. The values handed
//! back, and those given, are the caller's to wipe, through `Zeroize` or
//! `Zeroizing`. What the compiler copies on its own is beyond reach: a value
//! moved on the stack may leave a copy behind, and a register or a spill may
//! keep a word of it.

#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod big_uint;
mod fixed_big_uint;
mod inverse;
mod limb;
mod monty;
mod non_zero;
mod odd;
#[cfg(feature = "alloc")]
mod padded_big_uint;
mod traits;
mod wipe;

#[cfg(feature = "alloc")]
pub use big_uint::{BigMontyForm, BigMontyParams};
pub use fixed_big_uint::{FixedMontyForm, FixedMontyParams};
pub use non_zero::NonZero;
pub use odd::Odd;
#[cfg(feature = "alloc")]
pub use padded_big_uint::{PaddedMontyForm, PaddedMontyParams};
pub use traits::{ModAdd, ModInverse, ModMul, ModPow, ModSub};
