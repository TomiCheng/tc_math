//! The SEC 2 binary elliptic curves with each field fixed at compile time, after Bouncy
//! Castle's custom `SecT*` curves.
//!
//! Each field has its own element type, such as [`SecT283FieldElement`]: a fixed array of
//! `u64` limbs with the reduction by its trinomial or pentanomial written out. The curves are
//! those of `tc_f2m_curve`, whose field is a run-time value; fixing it at compile time makes
//! the field and point arithmetic several times faster. `BENCHES.md`, beside the crate's
//! sources, has the measurements.
//!
//! Every curve comes as two sets of types, and the scalar decides which to use:
//!
//! - [`SecTCtCurve`] and [`SecTCtPoint`], for secret scalars: affine points multiplied by
//!   the Montgomery ladder, constant time in the scalar. Only that multiplication is constant
//!   time; the additions and doublings are BC's affine formulas, variable time.
//! - [`SecTVarCurve`] and [`SecTVarPoint`], for public scalars only: lambda-projective points
//!   multiplied as BC's are, by the wNAF or, on the Koblitz curves, the window τ-adic NAF.
//!   [`SecTVarPrecomputed`] keeps that multiplication's table for a base used many times.
//!
//! [`named_curves`] holds the SEC 2 curves from sect163k1 to sect571r1, each of which builds
//! either curve and its generator. The `small-curves` feature adds sect113r1, sect113r2,
//! sect131r1 and sect131r2, which are too weak for new keys and serve interoperability only.
//!
//! # Timing
//!
//! Every item states whether it is constant time or variable time. On x86 the field product
//! runs on PCLMULQDQ and, on x86-64, squares and square roots run on BMI2's PDEP and PEXT,
//! each picked at run time when the CPU has the instruction; these paths are constant time.
//! Elsewhere the portable fallback, BC's scalar code, multiplies by indexing a small table
//! with bits of the operands: a cache-timing side channel under every constant-time claim
//! that rests on a field product. Squaring is constant time everywhere. Field elements wipe
//! their limbs on drop.
//!
//! The crate is `no_std` but needs an allocator: scalars are `PaddedBigUint`, encodings are
//! vectors, and the multipliers keep their tables on the heap.
//!
//! # Example
//!
//! ```
//! use tc_bigint::{ArrayEncoding, PaddedBigUint};
//! use tc_f2m_custom::named_curves::SECT283K1;
//!
//! // The constant-time curve, its generator and a secret scalar below the order.
//! let curve = SECT283K1.ct_curve();
//! let g = SECT283K1.ct_generator(&curve);
//! let Ok(k) = <PaddedBigUint as ArrayEncoding<u8>>::from_be(&[0x5a; 35]);
//!
//! // kG in the compressed SEC 1 encoding: a tag byte and the 36 bytes of x.
//! let public_key = g.multiply(&k).encode(true);
//! assert_eq!(public_key.len(), 37);
//! ```

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

extern crate alloc;

mod interleave;
mod sec1;
mod sect113_field;
mod sect113_field_element;
mod sect131_field;
mod sect131_field_element;
mod sect163_field;
mod sect163_field_element;
mod sect193_field;
mod sect193_field_element;
mod sect233_field;
mod sect233_field_element;
mod sect239_field;
mod sect239_field_element;
mod sect283_field;
mod sect283_field_element;
mod sect409_field;
mod sect409_field_element;
mod sect571_field;
mod sect571_field_element;
mod sect_ct_curve;
mod sect_ct_point;
mod sect_element;
mod sect_error;
mod sect_var_curve;
mod sect_var_point;
mod sect_var_precomputed;

pub mod named_curves;

pub use sect_ct_curve::SecTCtCurve;
pub use sect_ct_point::SecTCtPoint;
pub use sect_error::SecTError;
pub use sect_var_curve::SecTVarCurve;
pub use sect_var_point::SecTVarPoint;
pub use sect_var_precomputed::SecTVarPrecomputed;
pub use sect113_field_element::SecT113FieldElement;
pub use sect131_field_element::SecT131FieldElement;
pub use sect163_field_element::SecT163FieldElement;
pub use sect193_field_element::SecT193FieldElement;
pub use sect233_field_element::SecT233FieldElement;
pub use sect239_field_element::SecT239FieldElement;
pub use sect283_field_element::SecT283FieldElement;
pub use sect409_field_element::SecT409FieldElement;
pub use sect571_field_element::SecT571FieldElement;
