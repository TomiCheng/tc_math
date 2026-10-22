//! Prime-field short-Weierstrass elliptic curves `y² = x³ + ax + b`, after Bouncy Castle's
//! `FpCurve`, on the Montgomery arithmetic of `tc_modular`, with the field chosen at run time.
//!
//! [`FpField`] is the prime field and [`FpFieldElement`] an element of it. A curve over it
//! comes in two sets, and the caller picks one by the curve type:
//!
//! - [`FpCtCurve`] and [`FpCtPoint`]: the complete formulas of Renes, Costello and Batina
//!   and a 4-bit fixed window, constant time in the points and the scalar apart from the
//!   `bool` and `Option` answers. They serve wherever a scalar or a point is secret.
//! - [`FpVarCurve`] and [`FpVarPoint`]: BC's formulas in affine, homogeneous or Jacobian
//!   coordinates, wNAF, and GLV on a curve given one. They are variable time, only for
//!   public values, as when a signature is verified.
//!
//! [`named_curves`] holds the SEC 2 parameters of P-256 and secp256k1 and, behind the
//! `small-curves` feature, of the weak secp112r1, secp112r2 and secp128r2, which serve
//! interoperability only. [`FpCtPrecomputed`] and [`FpVarPrecomputed`] are the
//! [`BaseMultiplier`](tc_ec_core::multiplier::BaseMultiplier)s that keep a base's table for
//! the curve's own multiplication. The traits these types implement are in `tc_ec_core`.
//!
//! The crate needs an allocator, as field elements and tables live on the heap.
//!
//! ```
//! use tc_bigint::{ArrayEncoding, PaddedBigUint};
//! use tc_fp_curve::named_curves::SECP256R1;
//!
//! let field = SECP256R1.field();
//! let curve = SECP256R1.ct_curve(&field);
//! let g = SECP256R1.ct_generator(&curve);
//!
//! // A secret scalar as wide as the order, so the windows follow the width alone.
//! let k = <PaddedBigUint as ArrayEncoding<u8>>::from_be(&[0x5a; 32]).unwrap();
//! let public_key = g.multiply(&k);
//! let encoded = public_key.encode(true);
//! assert_eq!(encoded.len(), 33);
//! ```

#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
extern crate alloc;

mod fp_ct_curve;
mod fp_ct_point;
mod fp_ct_precomputed;
mod fp_error;
mod fp_field;
mod fp_field_element;
mod fp_var_curve;
mod fp_var_point;
mod fp_var_precomputed;
mod sec1;

pub mod named_curves;

pub use fp_ct_curve::FpCtCurve;
pub use fp_ct_point::FpCtPoint;
pub use fp_ct_precomputed::FpCtPrecomputed;
pub use fp_error::FpError;
pub use fp_field::FpField;
pub use fp_field_element::FpFieldElement;
pub use fp_var_curve::FpVarCurve;
pub use fp_var_point::FpVarPoint;
pub use fp_var_precomputed::FpVarPrecomputed;
