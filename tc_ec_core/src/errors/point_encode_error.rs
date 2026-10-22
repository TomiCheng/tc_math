//! The error of SEC 1 point encoding.

/// Why a SEC 1 point encoding could not be written.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PointEncodeError {
    /// The output is shorter than the encoding.
    OutputTooShort {
        /// The bytes the encoding takes.
        required: usize,
        /// The bytes the output has.
        available: usize,
    },
}

/// Constant time; the lengths are public.
impl core::fmt::Display for PointEncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutputTooShort {
                required,
                available,
            } => write!(
                f,
                "point encoding requires {required} bytes, but output has {available}"
            ),
        }
    }
}

impl core::error::Error for PointEncodeError {}
