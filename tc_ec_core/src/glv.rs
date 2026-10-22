//! BC's `GlvMultiplier`, `EndoUtilities.DecomposeScalar` and `ScalarSplitParameters`: on a
//! curve with an efficient endomorphism φ(P) = λP, kP splits into aP + bφ(P) with a and b
//! each about half as long as n, run together by interleaved wNAF. The caller gives φ, so
//! this crate needs no curve's β. Variable time, only for public scalars.
//!
//! As in BC, the result holds only for points of the subgroup of order n, where
//! φ(P) = λP holds.

mod mul;
mod split;

pub use mul::GlvTable;
pub use mul::glv_mul;
pub use mul::glv_mul_point;
pub use split::ScalarSplit;
