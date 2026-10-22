//! The SEC and SM2 prime curves with their fields fixed at compile time, after Bouncy Castle's
//! `SecP*` and `SM2P256V1` custom curves.
//!
//! Each field has its own element type, such as [`SecP256R1FieldElement`], on a fixed array of
//! limbs, with BC's Solinas or Mersenne reduction for its prime and safegcd for inversion, and
//! its arithmetic allocates nothing. Points therefore multiply about three times as fast as on
//! the generic `tc_fp_curve`, which takes its prime at run time; `BENCHES.md` has the numbers.
//!
//! Each curve comes in two sets, as in `tc_fp_curve`, and the values decide which to take:
//!
//! - [`SecPCtCurve`] and [`SecPCtPoint`]: constant time, for secret scalars and points, as in
//!   key generation, ECDH and signing. The complete formulas of Renes, Costello and Batina in
//!   homogeneous coordinates, and a 4-bit fixed window whose lookup reads the whole table.
//! - [`SecPVarCurve`] and [`SecPVarPoint`]: variable time, only for public values, as in
//!   verifying a signature. BC's Jacobian formulas and wNAF, and the GLV split on the k1
//!   curves.
//!
//! [`named_curves`] holds the domain parameters, which build both sets: secp192k1 to
//! secp521r1 and sm2p256v1, and with the `small-curves` feature the weak secp128r1, secp160k1,
//! secp160r1 and secp160r2, for interoperability only. [`SecPCtPrecomputed`] and
//! [`SecPVarPrecomputed`] keep each set's own multiplication table for a base used many times;
//! they implement `tc_ec_core`'s `BaseMultiplier`, as its comb does.
//!
//! Every item states whether it is constant time or variable time. The field elements are
//! constant time apart from their `bool` and `Option` answers, except the square root of
//! secp224r1, which is variable time.
//!
//! The crate needs an allocator: its scalars are `PaddedBigUint`s, and its multiplication
//! tables and encodings live in vectors.
//!
//! # Example
//!
//! ```
//! use tc_bigint::{ArrayEncoding, PaddedBigUint};
//! use tc_fp_custom::named_curves::SECP256R1;
//!
//! let curve = SECP256R1.ct_curve();
//! let generator = SECP256R1.ct_generator(&curve);
//!
//! // A secret scalar below the order; its width is public, so it is as wide as the order.
//! let Ok(secret) = PaddedBigUint::from_be(&[0x5a_u8; 32]);
//! let public = generator.multiply(&secret);
//! let encoded = public.encode(true);
//! assert_eq!(encoded.len(), 33);
//! ```

#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

extern crate alloc;

mod mod_inverse;
mod nat;
mod nat64;
mod sec1;
mod secp128r1_field;
mod secp128r1_field_element;
mod secp160r1_field;
mod secp160r1_field_element;
mod secp160r2_field;
mod secp160r2_field_element;
mod secp192k1_field;
mod secp192k1_field_element;
mod secp192r1_field;
mod secp192r1_field_element;
mod secp224k1_field;
mod secp224k1_field_element;
mod secp224r1_field;
mod secp224r1_field_element;
mod secp256k1_field;
mod secp256k1_field_element;
mod secp256r1_field;
mod secp256r1_field_element;
mod secp384r1_field;
mod secp384r1_field_element;
mod secp521r1_field;
mod secp521r1_field_element;
mod secp_ct_curve;
mod secp_ct_point;
mod secp_ct_precomputed;
mod secp_element;
mod secp_error;
mod secp_var_curve;
mod secp_var_point;
mod secp_var_precomputed;
mod sm2p256v1_field;
mod sm2p256v1_field_element;

pub mod named_curves;

pub use secp_ct_curve::SecPCtCurve;
pub use secp_ct_point::SecPCtPoint;
pub use secp_ct_precomputed::SecPCtPrecomputed;
pub use secp_error::SecPError;
pub use secp_var_curve::SecPVarCurve;
pub use secp_var_point::SecPVarPoint;
pub use secp_var_precomputed::SecPVarPrecomputed;
pub use secp128r1_field_element::SecP128R1FieldElement;
pub use secp160r1_field_element::SecP160R1FieldElement;
pub use secp160r2_field_element::SecP160R2FieldElement;
pub use secp192k1_field_element::SecP192K1FieldElement;
pub use secp192r1_field_element::SecP192R1FieldElement;
pub use secp224k1_field_element::SecP224K1FieldElement;
pub use secp224r1_field_element::SecP224R1FieldElement;
pub use secp256k1_field_element::SecP256K1FieldElement;
pub use secp256r1_field_element::SecP256R1FieldElement;
pub use secp384r1_field_element::SecP384R1FieldElement;
pub use secp521r1_field_element::SecP521R1FieldElement;
pub use sm2p256v1_field_element::SM2P256V1FieldElement;
