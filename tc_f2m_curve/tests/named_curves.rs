// The named curves of Bouncy Castle's tables other than SEC 2: each holds together, with G on
// the curve, n·G at infinity and Bouncy Castle's cofactor, and where Bouncy Castle gives G as a
// SEC 1 encoding, the curve decodes it to the same G. The parameters come from Bouncy Castle's
// source, and its compressed encodings were decompressed apart from this crate, so the decoding
// here checks them.

use tc_bigint::PaddedBigUint;
use tc_ec_core::{Curve, Point};
use tc_f2m_curve::F2mVarCurve;
use tc_f2m_curve::named_curves::*;

fn bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

// Bouncy Castle gives the DSTU 4145 base points as coordinates, so those have no encoding to
// decode.
fn check<const N: usize>(named: &F2mNamedCurve<N>, cofactor: u32, encoded: Option<&str>) {
    let ct_curve = named.ct_curve();
    let var_curve = named.var_curve();
    let g = named.ct_generator(&ct_curve);
    let h = named.var_generator(&var_curve);
    let n = ct_curve.order().unwrap();

    assert!(g.is_valid() && h.is_valid());
    assert!(g.multiply(n).is_infinity());
    assert!(<&F2mVarCurve as Curve>::multiply(&h, n).is_infinity());
    assert_eq!(ct_curve.cofactor(), Some(&PaddedBigUint::from(cofactor)));
    if let Some(encoded) = encoded {
        assert_eq!(ct_curve.decode_point(&bytes(encoded)), Ok(g.clone()));
        assert_eq!(var_curve.decode_point(&bytes(encoded)), Ok(h.clone()));
    }
    assert_eq!(g.twice().encode(true), h.twice().encode(true));
}

#[test]
fn c2pnb163v1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2PNB163V1,
        2,
        Some("0307af69989546103d79329fcc3d74880f33bbe803cb"),
    );
}

#[test]
fn c2pnb163v2_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2PNB163V2,
        2,
        Some("030024266e4eb5106d0a964d92c4860e2671db9b6cc5"),
    );
}

#[test]
fn c2pnb163v3_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2PNB163V3,
        2,
        Some("0202f9f87b7c574d0bdecf8a22e6524775f98cdebdcb"),
    );
}

#[test]
fn c2pnb176w1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2PNB176W1,
        65390,
        Some("038d16c2866798b600f9f08bb4a8e860f3298ce04a5798"),
    );
}

#[test]
fn c2tnb191v1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2TNB191V1,
        2,
        Some("0236b3daf8a23206f9c4f299d7b21a9c369137f2c84ae1aa0d"),
    );
}

#[test]
fn c2tnb191v2_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2TNB191V2,
        4,
        Some("023809b2b7cc1b28cc5a87926aad83fd28789e81e2c9e3bf10"),
    );
}

#[test]
fn c2tnb191v3_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2TNB191V3,
        6,
        Some("03375d4ce24fde434489de8746e71786015009e66e38a926dd"),
    );
}

#[test]
fn c2pnb208w1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2PNB208W1,
        65096,
        Some("0289fdfbe4abe193df9559ecf07ac0ce78554e2784eb8c1ed1a57a"),
    );
}

#[test]
fn c2tnb239v1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2TNB239V1,
        4,
        Some("0257927098fa932e7c0a96d3fd5b706ef7e5f5c156e16b7e7c86038552e91d"),
    );
}

#[test]
fn c2tnb239v2_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2TNB239V2,
        6,
        Some("0228f9d04e900069c8dc47a08534fe76d2b900b7d7ef31f5709f200c4ca205"),
    );
}

#[test]
fn c2tnb239v3_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2TNB239V3,
        10,
        Some("0370f6e9d04d289c4e89913ce3530bfde903977d42b146d539bf1bde4e9c92"),
    );
}

#[test]
fn c2pnb272w1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2PNB272W1,
        65286,
        Some("026108babb2ceebcf787058a056cbe0cfe622d7723a289e08a07ae13ef0d10d171dd8d"),
    );
}

#[test]
fn c2pnb304w1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2PNB304W1,
        65070,
        Some("02197b07845e9be2d96adb0f5f3c7f2cffbd7a3eb8b6fec35c7fd67f26ddf6285a644f740a2614"),
    );
}

#[test]
fn c2tnb359v1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2TNB359V1,
        76,
        Some(
            "033c258ef3047767e7ede0f1fdaa79daee3841366a132e163aced4ed2401df9c6bdcde98e8e707c07a2239b1b097",
        ),
    );
}

#[test]
fn c2pnb368w1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2PNB368W1,
        65392,
        Some(
            "021085e2755381dccce3c1557afa10c2f0c0c2825646c5b34a394cbcfa8bc16b22e7e789e927be216f02e1fb136a5f",
        ),
    );
}

#[test]
fn c2tnb431r1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &C2TNB431R1,
        10080,
        Some(
            "02120fc05d3c67a99de161d2f4092622feca701be4f50f4758714e8a87bbf2a658ef8c21e7c5efe965361f6c2999c0c247b0dbd70ce6b7",
        ),
    );
}

#[test]
fn dstu4145_163_holds_together() {
    check(&DSTU4145_163, 2, None);
}

#[test]
fn dstu4145_167_holds_together() {
    check(&DSTU4145_167, 2, None);
}

#[test]
fn dstu4145_173_holds_together() {
    check(&DSTU4145_173, 4, None);
}

#[test]
fn dstu4145_179_holds_together() {
    check(&DSTU4145_179, 2, None);
}

#[test]
fn dstu4145_191_holds_together() {
    check(&DSTU4145_191, 2, None);
}

#[test]
fn dstu4145_233_holds_together() {
    check(&DSTU4145_233, 2, None);
}

#[test]
fn dstu4145_257_holds_together() {
    check(&DSTU4145_257, 4, None);
}

#[test]
fn dstu4145_307_holds_together() {
    check(&DSTU4145_307, 2, None);
}

#[test]
fn dstu4145_367_holds_together() {
    check(&DSTU4145_367, 2, None);
}

#[test]
fn dstu4145_431_holds_together() {
    check(&DSTU4145_431, 2, None);
}
