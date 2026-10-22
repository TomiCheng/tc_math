use core::fmt;

use tc_bigint::PaddedBigUint;
use tc_ec_core::{CoordinateSystem, Curve, FieldElement, PointDecodeError, PrimeFieldElement};

use crate::{FpCtPoint, FpError, FpField, FpFieldElement, sec1};

/// `y² = x³ + ax + b` over a prime field, whose points take only constant-time
/// formulas: the complete formulas of Renes, Costello and Batina (2016) in homogeneous
/// coordinates. They are complete when the group has odd order, as it has on every
/// prime-order curve.
pub struct FpCtCurve<'a> {
    field: &'a FpField,
    a: FpFieldElement<'a>,
    b: FpFieldElement<'a>,
    b3: FpFieldElement<'a>,
    order: Option<PaddedBigUint>,
    cofactor: Option<PaddedBigUint>,
}

impl<'a> FpCtCurve<'a> {
    /// The curve over `field`, its order and cofactor taken as given. Fails when `a` or `b`
    /// is not below `q`. Constant time, apart from whether it fails.
    pub fn new(
        field: &'a FpField,
        a: &PaddedBigUint,
        b: &PaddedBigUint,
        order: Option<PaddedBigUint>,
        cofactor: Option<PaddedBigUint>,
    ) -> Result<Self, FpError> {
        let b = field.element(b)?;
        Ok(Self {
            field,
            a: field.element(a)?,
            b3: b.double().add(&b),
            b,
            order,
            cofactor,
        })
    }

    /// The field the curve is over. Constant time.
    pub fn field(&self) -> &'a FpField {
        self.field
    }

    /// The coefficient `a`. Constant time.
    pub fn a(&self) -> &FpFieldElement<'a> {
        &self.a
    }

    /// The coefficient `b`. Constant time.
    pub fn b(&self) -> &FpFieldElement<'a> {
        &self.b
    }

    pub(crate) fn b3(&self) -> &FpFieldElement<'a> {
        &self.b3
    }

    /// The order n of the subgroup, or `None` when unknown. Constant time.
    pub fn order(&self) -> Option<&PaddedBigUint> {
        self.order.as_ref()
    }

    /// The cofactor, or `None` when unknown. Constant time.
    pub fn cofactor(&self) -> Option<&PaddedBigUint> {
        self.cofactor.as_ref()
    }

    /// The point at infinity, `(0 : 1 : 0)`. Constant time.
    pub fn infinity(&'a self) -> FpCtPoint<'a> {
        FpCtPoint::infinity(self)
    }

    /// The point `(x, y)`, which is not checked against the curve; see
    /// [`FpCtPoint::is_valid`](tc_ec_core::Point::is_valid). Fails when a coordinate is not
    /// below `q`. Constant time, apart from whether it fails.
    pub fn create_point(
        &'a self,
        x: &PaddedBigUint,
        y: &PaddedBigUint,
    ) -> Result<FpCtPoint<'a>, FpError> {
        Ok(FpCtPoint::new(
            self,
            self.field.element(x)?,
            self.field.element(y)?,
        ))
    }

    /// The bytes of one coordinate in the SEC 1 encoding. Constant time.
    pub fn field_element_encoding_length(&self) -> usize {
        sec1::coordinate_length(self.field)
    }

    /// The bytes of the SEC 1 encoding of a finite point. Constant time.
    pub fn affine_point_encoding_length(&self, compressed: bool) -> usize {
        sec1::point_length(self.field, compressed)
    }

    /// Decodes the SEC 1 infinity, compressed, uncompressed and hybrid forms, and checks
    /// that the point is on the curve; the subgroup is not checked, as in BC. Variable
    /// time: only for a public encoding, as decompression takes a square root.
    pub fn decode_point(&'a self, encoded: &[u8]) -> Result<FpCtPoint<'a>, PointDecodeError> {
        Ok(match sec1::decode(self.field, &self.a, &self.b, encoded)? {
            None => self.infinity(),
            Some((x, y)) => FpCtPoint::new(self, x, y),
        })
    }
}

/// Constant time.
impl<'a> Curve for &'a FpCtCurve<'a> {
    type Field = FpFieldElement<'a>;
    type Point = FpCtPoint<'a>;
    type Scalar = PaddedBigUint;

    fn a(&self) -> &FpFieldElement<'a> {
        &self.a
    }

    fn b(&self) -> &FpFieldElement<'a> {
        &self.b
    }

    fn order(&self) -> Option<&PaddedBigUint> {
        self.order.as_ref()
    }

    fn cofactor(&self) -> Option<&PaddedBigUint> {
        self.cofactor.as_ref()
    }

    fn zero(&self) -> FpFieldElement<'a> {
        self.field.zero()
    }

    fn one(&self) -> FpFieldElement<'a> {
        self.field.one()
    }

    fn infinity(&self) -> FpCtPoint<'a> {
        FpCtPoint::infinity(self)
    }

    fn create_point(&self, x: FpFieldElement<'a>, y: FpFieldElement<'a>) -> FpCtPoint<'a> {
        FpCtPoint::new(self, x, y)
    }

    fn coordinate_system(&self) -> CoordinateSystem {
        CoordinateSystem::Homogeneous
    }

    /// The fixed-window multiplication of [`FpCtPoint::multiply`]. Constant time.
    fn multiply(point: &FpCtPoint<'a>, scalar: &PaddedBigUint) -> FpCtPoint<'a> {
        point.multiply(scalar)
    }
}

/// Same field and coefficients, as in BC; the order and cofactor are left out. Variable
/// time: only for public curves, as the comparison stops at the first difference.
impl PartialEq for FpCtCurve<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field && self.a == other.a && self.b == other.b
    }
}

impl Eq for FpCtCurve<'_> {}

/// Prints the field and the coefficients. Variable time: only for public values, as the
/// parameters of a curve are.
impl fmt::Debug for FpCtCurve<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FpCtCurve")
            .field("field", self.field)
            .field("a", &format_args!("{:#x}", self.a.to_big_uint()))
            .field("b", &format_args!("{:#x}", self.b.to_big_uint()))
            .finish_non_exhaustive()
    }
}
