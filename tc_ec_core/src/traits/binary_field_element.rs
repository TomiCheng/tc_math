//! What only the elements of a binary field add: the trace and the half-trace. Solving
//! `z² + z = β` needs the curve's coefficients and point-decompression rules, so it lives
//! on the curves, as in BC.

use super::FieldElement;

/// An element of a binary field F2m. Each implementation states the timing of its
/// operations.
pub trait BinaryFieldElement: FieldElement {
    /// The absolute trace, 0 or 1.
    fn trace(&self) -> u8;

    /// The half-trace, for an odd m.
    fn half_trace(&self) -> Self;
}
