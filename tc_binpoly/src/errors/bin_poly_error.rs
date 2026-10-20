use core::fmt;

/// Error returned when constructing or combining binary polynomials.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinPolyError {
    /// The polynomial degree is below the minimum for this modulus shape.
    DegreeTooSmall {
        /// The smallest degree of the shape.
        minimum: usize,
        /// The degree given.
        actual: usize,
    },
    /// The polynomial degree exceeds `2^20`, or `4096` without `alloc`.
    DegreeTooLarge {
        /// The largest degree.
        maximum: usize,
        /// The degree given.
        actual: usize,
    },
    /// A trinomial tap did not satisfy `0 < k < n`.
    InvalidTrinomialTap {
        /// The degree.
        n: usize,
        /// The middle exponent.
        k: usize,
    },
    /// Pentanomial taps did not satisfy `0 < k1 < k2 < k3 < n`.
    InvalidPentanomialTaps {
        /// The degree.
        n: usize,
        /// The lowest middle exponent.
        k1: usize,
        /// The second middle exponent.
        k2: usize,
        /// The highest middle exponent.
        k3: usize,
    },
    /// A limb slice had the wrong length.
    InvalidLength {
        /// The limbs the field takes.
        expected: usize,
        /// The limbs given.
        actual: usize,
    },
    /// Bits at positions greater than or equal to the configured degree were set.
    UnreducedValue,
    /// The output buffer is shorter than the encoding.
    BufferTooSmall,
}

/// Variable time: only for public values, as an error is.
impl fmt::Display for BinPolyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DegreeTooSmall { minimum, actual } => {
                write!(f, "degree must be at least {minimum}, got {actual}")
            }
            Self::DegreeTooLarge { maximum, actual } => {
                write!(f, "degree must be at most {maximum}, got {actual}")
            }
            Self::InvalidTrinomialTap { n, k } => {
                write!(f, "trinomial tap must satisfy 0 < k < n, got n={n}, k={k}")
            }
            Self::InvalidPentanomialTaps { n, k1, k2, k3 } => write!(
                f,
                "pentanomial taps must satisfy 0 < k1 < k2 < k3 < n, got n={n}, taps=({k1}, {k2}, {k3})"
            ),
            Self::InvalidLength { expected, actual } => {
                write!(f, "expected {expected} limbs, got {actual}")
            }
            Self::UnreducedValue => {
                f.write_str("value has coefficients at or above the modulus degree")
            }
            Self::BufferTooSmall => f.write_str("the output buffer is too small"),
        }
    }
}

impl core::error::Error for BinPolyError {}
