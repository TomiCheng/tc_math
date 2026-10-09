//! Provable primes by the Shawe-Taylor routine.

use tc_digest::Digest;

use crate::{StError, StOutput};

/// Provable primes by the Shawe-Taylor routine of FIPS 186-4 C.6, as Bouncy
/// Castle runs it. Each implementation states whether it is constant time.
///
/// ```
/// use tc_bigint::{FixedBigUint, Word};
/// use tc_prime::ShaweTaylor;
/// use tc_sha::Sha256Digest;
///
/// type U256 = FixedBigUint<{ 256 / Word::BITS as usize }>;
///
/// let seed = 0x0123_4567_89ab_cdef_fedc_ba98_7654_3210_u128.to_be_bytes();
/// let output = U256::st_random_prime(&mut Sha256Digest::new(), 64, &seed).unwrap();
/// assert_eq!(*output.prime(), U256::from(0xa809_ea1a_e987_5467_u64));
/// assert_eq!(output.prime_gen_counter(), 9);
/// ```
pub trait ShaweTaylor: Sized {
    /// A provable prime of exactly `bits` bits, with the seed and the
    /// counter it ends at; the same digest, bits and seed always give the
    /// same prime. The digest is reset by each finalization. Fails when
    /// `bits` is below two, the seed is empty, a value of the routine does
    /// not fit the type, or no prime turns up within the counts the
    /// standard allows.
    fn st_random_prime<D: Digest + ?Sized>(
        digest: &mut D,
        bits: u32,
        seed: &[u8],
    ) -> Result<StOutput<Self>, StError>;
}
