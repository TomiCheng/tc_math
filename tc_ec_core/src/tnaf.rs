//! BC's `Tnaf` and `WTauNafMultiplier`: multiplication on a binary Koblitz curve by the
//! reduced width-4 τ-adic NAF. The caller gives τ(P) = (x², y²), so this crate needs no
//! binary field. Variable time, only for public scalars.
//!
//! As in BC, the result holds only for points of the subgroup of order n: the scalar is
//! first reduced modulo δ = (τ^m − 1) / (τ − 1), which vanishes on that subgroup alone.
//! A check that a point lies in the subgroup must not go through here.

mod digits;
mod reduction;
mod table;

pub use table::WTauNafTable;
pub use table::is_koblitz;
pub use table::wtnaf_mul;
pub use table::wtnaf_mul_point;

// BC's `Tnaf.Width`.
const WIDTH: u32 = 4;
// The precision c that BC's `WTauNafMultiplier` passes to `PartModReduction`.
const PRECISION: u32 = 10;
