//! What only the elements of a prime field add: the conversion to and from an unsigned
//! integer, whose type the implementation names, so that this crate needs no particular
//! big-integer type for it.

use super::FieldElement;

/// An element of a prime field Fp. Each implementation states the timing of its
/// conversions.
pub trait PrimeFieldElement: FieldElement {
    /// The unsigned integer the element converts to and from.
    type BigUint;

    /// The element of the same field for `value`, reduced modulo q.
    fn element_from_big_uint(&self, value: &Self::BigUint) -> Self;

    /// The element as an integer in `[0, q)`.
    fn to_big_uint(&self) -> Self::BigUint;
}
