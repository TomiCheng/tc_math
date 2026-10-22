use core::fmt;

/// A coordinate that a SEC binary curve cannot take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecTError {
    /// An encoded element is not `⌈m / 8⌉` bytes long.
    InvalidLength {
        /// The length of the field's encoding, `⌈m / 8⌉`.
        expected: usize,
        /// The length given.
        actual: usize,
    },
    /// A coordinate has a bit at or above the degree `m`.
    NotReduced,
}

/// Variable time: only for public values, as an error is.
impl fmt::Display for SecTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { expected, actual } => {
                write!(f, "expected {expected} bytes, got {actual}")
            }
            Self::NotReduced => f.write_str("a value has a bit at or above the degree"),
        }
    }
}

impl core::error::Error for SecTError {}
