//! Elliptic curves `y² + xy = x³ + ax² + b` over the binary fields GF(2^m), after Bouncy
//! Castle's `F2mCurve` and `F2mPoint`, on the fields of [`tc_binpoly`] and the traits of
//! [`tc_ec_core`].
//!
//! Every curve comes as two sets of types, and the caller picks the set by its scalars:
//!
//! - [`F2mCtCurve`] and [`F2mCtPoint`] multiply by a constant-time Montgomery ladder, for
//!   secret scalars, as in key generation, signing and ECDH. Their group operations are
//!   BC's affine formulas, variable time and only for public points.
//! - [`F2mVarCurve`] and [`F2mVarPoint`] are BC's own arithmetic, in affine, homogeneous or
//!   lambda-projective coordinates, multiplying by the window τ-adic NAF on Koblitz curves
//!   and by the wNAF elsewhere. Variable time, only for public scalars, as in verification.
//!
//! [`F2mVarPrecomputed`] is the Var curve's multiplication with its table kept, for a base
//! that is multiplied again and again, as a verifier's generator is. [`named_curves`] holds
//! the SEC 2 curves from sect163k1 to sect571r1; the weak sect113 and sect131 curves come
//! only with the `small-curves` feature, for interoperability.
//!
//! Every item states whether it is constant time or variable time. The field arithmetic is
//! tc_binpoly's: constant time on x86 with PCLMULQDQ; elsewhere its product indexes a table
//! by bits of the operands, a cache-timing side channel.
//!
//! The crate needs an allocator, as its field elements and scalars live on the heap.
//!
//! # Example
//!
//! ```
//! use tc_bigint::{ArrayEncoding, PaddedBigUint};
//! use tc_f2m_curve::named_curves::SECT283K1;
//!
//! let curve = SECT283K1.ct_curve();
//! let g = SECT283K1.ct_generator(&curve);
//! // The private key: a secret below the order, drawn at random in practice.
//! let private_key = PaddedBigUint::from_be(&[0x5a_u8; 35])?;
//! let public_key = g.multiply(&private_key);
//! let encoded = public_key.encode(true);
//! assert_eq!(encoded.len(), 1 + 36);
//! # Ok::<(), core::convert::Infallible>(())
//! ```

#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
extern crate alloc;

mod f2m_ct_curve;
mod f2m_ct_point;
mod f2m_error;
mod f2m_field;
mod f2m_field_element;
mod f2m_var_curve;
mod f2m_var_point;
mod f2m_var_precomputed;
mod sec1;

pub mod named_curves;

pub use f2m_ct_curve::F2mCtCurve;
pub use f2m_ct_point::F2mCtPoint;
pub use f2m_error::F2mError;
pub use f2m_field::F2mField;
pub use f2m_field_element::F2mFieldElement;
pub use f2m_var_curve::F2mVarCurve;
pub use f2m_var_point::F2mVarPoint;
pub use f2m_var_precomputed::F2mVarPrecomputed;
