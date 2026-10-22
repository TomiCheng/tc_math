use tc_bigint::{PaddedBigUint, Word};

/// Bit access to an unsigned scalar, as used by scalar recoding and multiplication.
///
/// Every method is variable time and only for public scalars.
pub trait Scalar: Clone {
    /// The number of significant bits; zero has none. Variable time: only for public
    /// values.
    fn bit_length(&self) -> usize;

    /// Whether bit `index` is set; bits past the length read as zero. Variable time: only
    /// for public values.
    fn test_bit(&self, index: usize) -> bool;

    /// Whether the scalar is zero. Variable time: only for public values.
    fn is_zero(&self) -> bool;

    /// The scalar shifted right by `count` bits. Variable time: only for public values.
    fn shr(&self, count: usize) -> Self;

    /// The low `width` bits, `self mod 2^width`, for `width` up to 32. Variable time: only
    /// for public values.
    fn low_bits(&self, width: usize) -> u32;
}

/// Variable time: only for public values, as every method of [`Scalar`] is.
impl Scalar for PaddedBigUint {
    fn bit_length(&self) -> usize {
        self.bits() as usize
    }

    fn test_bit(&self, index: usize) -> bool {
        index < width(self) && self.bit(index as u32)
    }

    fn is_zero(&self) -> bool {
        self.bits() == 0
    }

    fn shr(&self, count: usize) -> Self {
        if count >= width(self) {
            PaddedBigUint::from(0u8)
        } else {
            self >> count as u32
        }
    }

    fn low_bits(&self, width: usize) -> u32 {
        assert!(width <= 32, "at most 32 low bits");
        (0..width).fold(0, |bits, i| bits | u32::from(self.test_bit(i)) << i)
    }
}

fn width(value: &PaddedBigUint) -> usize {
    value.as_limbs().len() * Word::BITS as usize
}
