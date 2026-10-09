//! Primality testing and prime generation for cryptography, over the
//! integers of `tc_bigint` and the modular arithmetic of `tc_modular`,
//! with the random integers they draw on.
//!
//! [`Primality`] tests and generates probable primes as Bouncy Castle's
//! `Primes` does: trial division by the primes below 212, then rounds of
//! Miller-Rabin, or the enhanced test of FIPS 186-4 C.3.2, which tells a
//! composite apart and may find a factor of it. `ShaweTaylor`, behind the
//! `shawe-taylor` feature, generates provable primes by the routine of
//! FIPS 186-4 C.6 from a digest of `tc_digest` and a seed. Both are
//! implemented for `FixedBigUint<N>` and, with `alloc`, for `PaddedBigUint`
//! and `BigUint`. The random values come from a `rand_core` generator that
//! the caller gives.
//!
//! ```
//! use tc_bigint::{BitOps, FixedBigUint, Word};
//! use tc_prime::Primality;
//! # use core::convert::Infallible;
//! # struct Xorshift(u64);
//! # impl rand_core::TryRng for Xorshift {
//! #     type Error = Infallible;
//! #     fn try_next_u32(&mut self) -> Result<u32, Infallible> {
//! #         Ok(self.try_next_u64()? as u32)
//! #     }
//! #     fn try_next_u64(&mut self) -> Result<u64, Infallible> {
//! #         self.0 ^= self.0 << 13;
//! #         self.0 ^= self.0 >> 7;
//! #         self.0 ^= self.0 << 17;
//! #         Ok(self.0)
//! #     }
//! #     fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Infallible> {
//! #         for chunk in dst.chunks_mut(8) {
//! #             let word = self.try_next_u64()?.to_le_bytes();
//! #             chunk.copy_from_slice(&word[..chunk.len()]);
//! #         }
//! #         Ok(())
//! #     }
//! # }
//!
//! type U256 = FixedBigUint<{ 256 / Word::BITS as usize }>;
//!
//! // any generator of rand_core; a seeded one keeps the example repeatable
//! let mut rng = Xorshift(0x2545_f491_4f6c_dd1d);
//!
//! let p = U256::from(i128::MAX as u128); // 2^127 - 1, a prime
//! assert!(p.is_probable_prime(40, &mut rng));
//! assert!(!(&p * &p).is_probable_prime(40, &mut rng));
//!
//! let q = U256::random_probable_prime(&mut rng, 256, 40);
//! assert_eq!(q.bits(), 256);
//! ```
//!
//! # Timing
//!
//! Primality testing and generation are variable time over every integer
//! type, as Bouncy Castle's are: only for public values, or for candidates
//! whose timing a caller accepts showing, as key generation does. Every
//! method says which it is.

#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod big_uint;
mod fixed_big_uint;
mod generate;
mod miller_rabin;
mod mr_output;
#[cfg(feature = "alloc")]
mod padded_big_uint;
#[cfg(feature = "shawe-taylor")]
mod shawe_taylor;
mod small_factors;
#[cfg(feature = "shawe-taylor")]
mod st_error;
#[cfg(feature = "shawe-taylor")]
mod st_output;
#[cfg(test)]
mod testing;
mod traits;

pub use mr_output::MrOutput;
#[cfg(feature = "shawe-taylor")]
pub use st_error::StError;
#[cfg(feature = "shawe-taylor")]
pub use st_output::StOutput;
pub use traits::Primality;
#[cfg(feature = "shawe-taylor")]
pub use traits::ShaweTaylor;
