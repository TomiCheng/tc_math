//! The domain parameters of named prime-field curves, each an [`FpNamedCurve`] that builds its
//! Ct or Var curve and generator, from Bouncy Castle's tables:
//!
//! - `SecNamedCurves`: [`SECP256R1`], NIST P-256, and [`SECP256K1`]. The `small-curves`
//!   feature adds `SECP112R1`, `SECP112R2` and `SECP128R2`, which are too weak for new use and
//!   serve interoperability only.
//! - `TeleTrusTNamedCurves`: the Brainpool curves of RFC 5639, [`BRAINPOOLP160R1`] to
//!   [`BRAINPOOLP512T1`], each r1 curve with its t1 twin, where a = -3.
//! - `ECGost3410NamedCurves`: the GOST R 34.10 curves of RFC 4357 and RFC 7836. Their twelve
//!   names come to seven parameter sets, and each alias, such as
//!   [`TC26_GOST_3410_12_256_PARAMSETB`], is the constant it names.
//! - `X962NamedCurves`: [`PRIME192V2`], [`PRIME192V3`] and [`PRIME239V1`] to
//!   [`PRIME239V3`]; its prime192v1 and prime256v1 are SEC 2's secp192r1 and secp256r1.
//! - `ANSSINamedCurves`: [`FRP256V1`].
//! - `GMNamedCurves`: [`WAPIP192V1`] and its alias [`WAPI192V1`]; its sm2p256v1 is in
//!   `tc_fp_custom`.

mod anssi;
mod brainpool;
mod fp_named_curve;
mod gost;
mod hex;
#[cfg(feature = "small-curves")]
mod secp112r1;
#[cfg(feature = "small-curves")]
mod secp112r2;
#[cfg(feature = "small-curves")]
mod secp128r2;
mod secp256k1;
mod secp256r1;
#[cfg(test)]
mod tests;
mod wapi;
mod x962;

pub use anssi::FRP256V1;
pub use brainpool::{
    BRAINPOOLP160R1, BRAINPOOLP160T1, BRAINPOOLP192R1, BRAINPOOLP192T1, BRAINPOOLP224R1,
    BRAINPOOLP224T1, BRAINPOOLP256R1, BRAINPOOLP256T1, BRAINPOOLP320R1, BRAINPOOLP320T1,
    BRAINPOOLP384R1, BRAINPOOLP384T1, BRAINPOOLP512R1, BRAINPOOLP512T1,
};
pub use fp_named_curve::FpNamedCurve;
pub use gost::{
    GOSTR3410_2001_CRYPTOPRO_A, GOSTR3410_2001_CRYPTOPRO_B, GOSTR3410_2001_CRYPTOPRO_C,
    GOSTR3410_2001_CRYPTOPRO_XCHA, GOSTR3410_2001_CRYPTOPRO_XCHB, TC26_GOST_3410_12_256_PARAMSETA,
    TC26_GOST_3410_12_256_PARAMSETB, TC26_GOST_3410_12_256_PARAMSETC,
    TC26_GOST_3410_12_256_PARAMSETD, TC26_GOST_3410_12_512_PARAMSETA,
    TC26_GOST_3410_12_512_PARAMSETB, TC26_GOST_3410_12_512_PARAMSETC,
};
#[cfg(feature = "small-curves")]
pub use secp112r1::SECP112R1;
#[cfg(feature = "small-curves")]
pub use secp112r2::SECP112R2;
#[cfg(feature = "small-curves")]
pub use secp128r2::SECP128R2;
pub use secp256k1::SECP256K1;
pub use secp256r1::SECP256R1;
pub use wapi::{WAPI192V1, WAPIP192V1};
pub use x962::{PRIME192V2, PRIME192V3, PRIME239V1, PRIME239V2, PRIME239V3};
