//! BC's `ECMultiplier`, with the base fixed at construction: a multiplier holds the point
//! and its precomputed table, and `multiply` takes only the scalar. Only the consumer
//! knows whether a base will be reused, so the consumer picks the implementation, and
//! [`PerCall`] keeps a curve's own multiplication. The base often arrives with a key at
//! init, so the code that uses the multiplier, a signer for one, builds it with
//! [`BaseMultiplier::new`], and the consumer names only its type.

mod comb;

use crate::Curve;

pub use comb::{CombCt, CombVar};

/// Scalar multiplication of a fixed base. Each implementation states its timing.
pub trait BaseMultiplier<C: Curve> {
    /// The multiplier of `base`, its table built.
    fn new(base: C::Point) -> Self;

    /// `scalar` times the base.
    fn multiply(&self, scalar: &C::Scalar) -> C::Point;
}

/// No table kept: every multiplication calls the curve's own [`Curve::multiply`], which
/// builds its table each time.
pub struct PerCall<C: Curve> {
    point: C::Point,
}

/// Constant time where the curve's own `multiply` is, as on the Ct curves, and variable
/// time where it is not; `new` only keeps the point.
impl<C: Curve> BaseMultiplier<C> for PerCall<C> {
    fn new(point: C::Point) -> Self {
        Self { point }
    }

    fn multiply(&self, scalar: &C::Scalar) -> C::Point {
        C::multiply(&self.point, scalar)
    }
}
