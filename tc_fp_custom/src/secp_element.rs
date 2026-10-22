use tc_bigint::PaddedBigUint;
use tc_constant_time::{ConditionallySelectable, ConstantTimeEq};
use tc_ec_core::PrimeFieldElement;

/// What the curves need of a field element that has no field object to build it from.
/// Sealed: the module is private, so no other crate can name it.
pub trait SecPElement:
    PrimeFieldElement<BigUint = PaddedBigUint> + ConditionallySelectable + ConstantTimeEq
{
    /// The bytes of an encoded element, BC's `GetEncodedLength`.
    const LENGTH: usize;

    /// The additive identity.
    fn zero() -> Self;

    /// The multiplicative identity.
    fn one() -> Self;

    /// `None` when `bytes` are not the field's length or not below p.
    fn from_be(bytes: &[u8]) -> Option<Self>;

    /// The low bit of the value, the SEC 1 compression bit of y, in constant time.
    fn low_bit(&self) -> u8;
}
