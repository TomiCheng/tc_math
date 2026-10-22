//! BC's `WNafUtilities` and the sums of `ECAlgorithms`: NAF and wNAF recoding, the tables
//! of odd multiples, and wNAF multiplication of one point or of a sum of several. Variable
//! time, only for public scalars.

mod func;
mod mul;
mod table;

pub use func::generate_compact_window_naf;
pub use func::generate_jsf;
pub use func::generate_naf;
pub use func::generate_window_naf;
pub use func::get_naf_weight;
pub use func::get_window_size;
pub use func::wnaf_mul;
pub use func::wnaf_mul_point;
pub use mul::shamirs_trick;
pub use mul::sum_of_multiplies;
pub use mul::sum_of_two_multiplies;
pub use table::WNafTable;

// The default window-size cutoffs of BC's `WNafUtilities`.
const DEFAULT_WINDOW_SIZE_CUTOFFS: [usize; 6] = [13, 41, 121, 337, 897, 2305];
// A limit of the dense `Vec<i8>` form, not of BC's algorithm: a signed digit of a width
// past 8 does not fit an i8.
const MAX_DENSE_WIDTH: usize = 8;
// BC's `WNafUtilities.MAX_WIDTH`; a compact digit sits in the high 16 bits of an i32.
const MAX_COMPACT_WIDTH: usize = 16;
