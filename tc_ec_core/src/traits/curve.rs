//! The static link between a curve's parameters and its point type.
//!
//! Prime and binary curves share coefficients, an order and a cofactor, while their field
//! elements and point formulas differ. Associated types keep the algorithms generic and
//! let a curve specialized to one field, as BC's `custom/sec` curves are, bring its own
//! types without trait objects.

use super::FieldElement;
use super::Point;
use super::Scalar;
use crate::CoordinateSystem;

/// An elliptic curve, implemented on a handle to it such as `&Curve`. Each implementation
/// states the timing of its operations.
pub trait Curve: Clone {
    /// The elements of the curve's field.
    type Field: FieldElement;

    /// The curve's points.
    type Point: Point<Curve = Self>;

    /// The unsigned integer of scalars, the order and the cofactor.
    type Scalar: Scalar;

    /// The coefficient `a`.
    fn a(&self) -> &Self::Field;

    /// The coefficient `b`.
    fn b(&self) -> &Self::Field;

    /// The order n of the subgroup, or `None` when unknown.
    fn order(&self) -> Option<&Self::Scalar>;

    /// The cofactor, or `None` when unknown.
    fn cofactor(&self) -> Option<&Self::Scalar>;

    /// Zero in the curve's field.
    ///
    /// The curve, not [`FieldElement`], makes it, since a field chosen at run time takes
    /// its modulus or reduction from the curve.
    fn zero(&self) -> Self::Field;

    /// One in the curve's field.
    fn one(&self) -> Self::Field;

    /// The point at infinity.
    fn infinity(&self) -> Self::Point;

    /// The point of the affine coordinates `x` and `y`, already in the curve's field. It
    /// is not checked against the curve.
    fn create_point(&self, x: Self::Field, y: Self::Field) -> Self::Point;

    /// The coordinates the points are kept in.
    fn coordinate_system(&self) -> CoordinateSystem;

    /// `scalar · point`. There is no default: each curve picks its own multiplication
    /// and states its timing.
    fn multiply(point: &Self::Point, scalar: &Self::Scalar) -> Self::Point
    where
        Self: Sized;
}
