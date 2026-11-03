//! The named binary curves, each an [`F2mNamedCurve`] that builds its Ct or Var curve and
//! generator, from Bouncy Castle's tables:
//!
//! - `SecNamedCurves`: the binary curves of SEC 2, sect163k1 to sect571r1. The curves below
//!   160 bits, sect113r1, sect113r2, sect131r1 and sect131r2, are weak and come only with
//!   the `small-curves` feature, for interoperability.
//! - `X962NamedCurves`: the c2pnb and c2tnb curves of X9.62, [`C2PNB163V1`] to
//!   [`C2TNB431R1`].
//! - `Dstu4145NamedCurves`: the ten curves of the Ukrainian DSTU 4145-2002, which Bouncy
//!   Castle names only by their OIDs, 1.2.804.2.1.1.1.1.3.1.1.2.0 to .9. Here they go by their
//!   degree, [`DSTU4145_163`] to [`DSTU4145_431`].

mod dstu4145;
mod f2m_named_curve;
mod hex;
#[cfg(feature = "small-curves")]
mod sect113r1;
#[cfg(feature = "small-curves")]
mod sect113r2;
#[cfg(feature = "small-curves")]
mod sect131r1;
#[cfg(feature = "small-curves")]
mod sect131r2;
mod sect163k1;
mod sect163r1;
mod sect163r2;
mod sect193r1;
mod sect193r2;
mod sect233k1;
mod sect233r1;
mod sect239k1;
mod sect283k1;
mod sect283r1;
mod sect409k1;
mod sect409r1;
mod sect571k1;
mod sect571r1;
#[cfg(test)]
mod tests;
mod x962;

pub use dstu4145::{
    DSTU4145_163, DSTU4145_167, DSTU4145_173, DSTU4145_179, DSTU4145_191, DSTU4145_233,
    DSTU4145_257, DSTU4145_307, DSTU4145_367, DSTU4145_431,
};
pub use f2m_named_curve::F2mNamedCurve;
use f2m_named_curve::Taps;
#[cfg(feature = "small-curves")]
pub use sect113r1::SECT113R1;
#[cfg(feature = "small-curves")]
pub use sect113r2::SECT113R2;
#[cfg(feature = "small-curves")]
pub use sect131r1::SECT131R1;
#[cfg(feature = "small-curves")]
pub use sect131r2::SECT131R2;
pub use sect163k1::SECT163K1;
pub use sect163r1::SECT163R1;
pub use sect163r2::SECT163R2;
pub use sect193r1::SECT193R1;
pub use sect193r2::SECT193R2;
pub use sect233k1::SECT233K1;
pub use sect233r1::SECT233R1;
pub use sect239k1::SECT239K1;
pub use sect283k1::SECT283K1;
pub use sect283r1::SECT283R1;
pub use sect409k1::SECT409K1;
pub use sect409r1::SECT409R1;
pub use sect571k1::SECT571K1;
pub use sect571r1::SECT571R1;
pub use x962::{
    C2PNB163V1, C2PNB163V2, C2PNB163V3, C2PNB176W1, C2PNB208W1, C2PNB272W1, C2PNB304W1, C2PNB368W1,
    C2TNB191V1, C2TNB191V2, C2TNB191V3, C2TNB239V1, C2TNB239V2, C2TNB239V3, C2TNB359V1, C2TNB431R1,
};
