//! The group operations shared by the points of prime and binary curves.
//!
//! Their coordinate formulas differ, but infinity, addition, doubling, negation and
//! normalization mean the same. The associated curve type ties a point to its curve and
//! field, so generic algorithms dispatch statically.

use super::Curve;

/// A point of an elliptic curve. Each implementation states the timing of its operations.
pub trait Point: Clone + Eq {
    /// The curve the point is on.
    type Curve: Curve<Point = Self>;

    /// The curve of this point.
    fn curve(&self) -> Self::Curve;

    /// Whether the point satisfies the curve equation and, when the order is known, lies in
    /// the subgroup. Infinity is valid; a protocol that must refuse it does so itself.
    fn is_valid(&self) -> bool;

    /// The point at infinity of the same curve.
    fn infinity(&self) -> Self;

    /// Whether this is the point at infinity.
    fn is_infinity(&self) -> bool;

    /// The affine x coordinate, or `None` for infinity.
    fn x(&self) -> Option<<Self::Curve as Curve>::Field>;

    /// The affine y coordinate, or `None` for infinity.
    fn y(&self) -> Option<<Self::Curve as Curve>::Field>;

    /// The sum.
    fn add(&self, rhs: &Self) -> Self;

    /// The double.
    fn twice(&self) -> Self;

    /// `2P + Q`; an implementation may override this fallback.
    fn twice_plus(&self, rhs: &Self) -> Self {
        self.twice().add(rhs)
    }

    /// `3P`; an implementation may override this fallback.
    fn three_times(&self) -> Self {
        self.twice().add(self)
    }

    /// `2^exponent · P`; an implementation may override this fallback.
    fn times_pow2(&self, exponent: usize) -> Self {
        let mut result = self.clone();
        for _ in 0..exponent {
            result = result.twice();
        }
        result
    }

    /// The negation.
    fn negate(&self) -> Self;

    /// The same point in affine form.
    fn normalize(&self) -> Self;

    /// The projective scale Z, or `None` for infinity and affine points.
    fn projective_z(&self) -> Option<<Self::Curve as Curve>::Field>;

    /// The point in affine form, given the inverse of
    /// [`projective_z`](Self::projective_z) in the same field, which is not checked. No
    /// inversion is performed.
    fn normalize_with_inverse(&self, inverse: &<Self::Curve as Curve>::Field) -> Self;
}
