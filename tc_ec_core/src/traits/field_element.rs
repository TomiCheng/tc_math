//! The operations shared by the elements of prime and binary fields.
//!
//! Both kinds add, subtract, multiply, square, take square roots and invert. A
//! prime field may have no square root for an element, so `sqrt` answers with an
//! `Option`. Zero and one come from the curve, [`Curve::zero`](crate::Curve::zero)
//! and [`Curve::one`](crate::Curve::one), since a field chosen at run time takes its
//! modulus or reduction from the curve. The trace and half-trace, which only binary
//! fields have, live in [`BinaryFieldElement`](crate::BinaryFieldElement).

/// An element of a prime or binary field. Each implementation states the timing of its
/// operations.
pub trait FieldElement: Clone + Eq {
    /// Whether this is the additive identity.
    fn is_zero(&self) -> bool;

    /// Whether this is the multiplicative identity.
    fn is_one(&self) -> bool;

    /// The sum.
    fn add(&self, rhs: &Self) -> Self;

    /// The difference.
    fn subtract(&self, rhs: &Self) -> Self;

    /// The product.
    fn multiply(&self, rhs: &Self) -> Self;

    /// The square.
    fn square(&self) -> Self;

    /// A square root, when there is one.
    fn sqrt(&self) -> Option<Self>;

    /// The additive inverse.
    fn negate(&self) -> Self;

    /// The multiplicative inverse; zero has none.
    fn invert(&self) -> Option<Self>;

    /// The length of SEC 1 FE2OSP, as BC's `GetEncodedLength`: the field's bits divided
    /// by eight, rounded up.
    fn encoded_length(&self) -> usize;

    /// SEC 1 FE2OSP, as BC's `EncodeTo`: the element, big-endian, into `output` of exactly
    /// [`encoded_length`](Self::encoded_length) bytes. Panics on any other length. ECDH
    /// takes its shared secret and ECDSA its r from the x coordinate this way.
    fn encode_to(&self, output: &mut [u8]);
}
