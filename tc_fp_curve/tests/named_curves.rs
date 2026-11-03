// The named curves of Bouncy Castle's tables other than SEC 2: each holds together, with G on
// the curve, n·G at infinity and Bouncy Castle's cofactor, and where Bouncy Castle gives G as a
// SEC 1 encoding, the curve decodes it to the same G. The parameters come from Bouncy Castle's
// source, and its compressed encodings were decompressed apart from this crate, so the decoding
// here checks them.

use tc_bigint::PaddedBigUint;
use tc_ec_core::Point;
use tc_fp_curve::named_curves::*;

fn bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn check<const N: usize>(named: &FpNamedCurve<N>, cofactor: u32, encoded: &str) {
    let field = named.field();
    let ct_curve = named.ct_curve(&field);
    let var_curve = named.var_curve(&field);
    let g = named.ct_generator(&ct_curve);
    let h = named.var_generator(&var_curve);
    let n = ct_curve.order().unwrap();

    assert!(g.is_valid() && h.is_valid());
    assert!(g.multiply(n).is_infinity() && h.multiply(n).is_infinity());
    assert_eq!(ct_curve.cofactor(), Some(&PaddedBigUint::from(cofactor)));
    assert_eq!(ct_curve.decode_point(&bytes(encoded)), Ok(g.clone()));
    assert_eq!(var_curve.decode_point(&bytes(encoded)), Ok(h.clone()));
    assert_eq!(g.twice().encode(true), h.twice().encode(true));
}

#[test]
fn brainpoolp160r1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP160R1,
        1,
        "04bed5af16ea3f6a4f62938c4631eb5af7bdbcdbc31667cb477a1a8ec338f94741669c976316da6321",
    );
}

#[test]
fn brainpoolp160t1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP160T1,
        1,
        "04b199b13b9b34efc1397e64baeb05acc265ff2378add6718b7c7c1961f0991b842443772152c9e0ad",
    );
}

#[test]
fn brainpoolp192r1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP192R1,
        1,
        "04c0a0647eaab6a48753b033c56cb0f0900a2f5c4853375fd614b690866abd5bb88b5f4828c1490002e6773fa2fa299b8f",
    );
}

#[test]
fn brainpoolp192t1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP192T1,
        1,
        "043ae9e58c82f63c30282e1fe7bbf43fa72c446af6f4618129097e2c5667c2223a902ab5ca449d0084b7e5b3de7ccc01c9",
    );
}

#[test]
fn brainpoolp224r1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP224R1,
        1,
        "040d9029ad2c7e5cf4340823b2a87dc68c9e4ce3174c1e6efdee12c07d58aa56f772c0726f24c6b89e4ecdac24354b9e99caa3f6d3761402cd",
    );
}

#[test]
fn brainpoolp224t1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP224T1,
        1,
        "046ab1e344ce25ff3896424e7ffe14762ecb49f8928ac0c76029b4d5800374e9f5143e568cd23f3f4d7c0d4b1e41c8cc0d1c6abd5f1a46db4c",
    );
}

#[test]
fn brainpoolp256r1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP256R1,
        1,
        "048bd2aeb9cb7e57cb2c4b482ffc81b7afb9de27e1e3bd23c23a4453bd9ace3262547ef835c3dac4fd97f8461a14611dc9c27745132ded8e545c1d54c72f046997",
    );
}

#[test]
fn brainpoolp256t1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP256T1,
        1,
        "04a3e8eb3cc1cfe7b7732213b23a656149afa142c47aafbc2b79a191562e1305f42d996c823439c56d7f7b22e14644417e69bcb6de39d027001dabe8f35b25c9be",
    );
}

#[test]
fn brainpoolp320r1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP320R1,
        1,
        "0443bd7e9afb53d8b85289bcc48ee5bfe6f20137d10a087eb6e7871e2a10a599c710af8d0d39e2061114fdd05545ec1cc8ab4093247f77275e0743ffed117182eaa9c77877aaac6ac7d35245d1692e8ee1",
    );
}

#[test]
fn brainpoolp320t1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP320T1,
        1,
        "04925be9fb01afc6fb4d3e7d4990010f813408ab106c4f09cb7ee07868cc136fff3357f624a21bed5263ba3a7a27483ebf6671dbef7abb30ebee084e58a0b077ad42a5a0989d1ee71b1b9bc0455fb0d2c3",
    );
}

#[test]
fn brainpoolp384r1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP384R1,
        1,
        "041d1c64f068cf45ffa2a63a81b7c13f6b8847a3e77ef14fe3db7fcafe0cbd10e8e826e03436d646aaef87b2e247d4af1e8abe1d7520f9c2a45cb1eb8e95cfd55262b70b29feec5864e19c054ff99129280e4646217791811142820341263c5315",
    );
}

#[test]
fn brainpoolp384t1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP384T1,
        1,
        "0418de98b02db9a306f2afcd7235f72a819b80ab12ebd653172476fecd462aabffc4ff191b946a5f54d8d0aa2f418808cc25ab056962d30651a114afd2755ad336747f93475b7a1fca3b88f2b6a208ccfe469408584dc2b2912675bf5b9e582928",
    );
}

#[test]
fn brainpoolp512r1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP512R1,
        1,
        "0481aee4bdd82ed9645a21322e9c4c6a9385ed9f70b5d916c1b43b62eef4d0098eff3b1f78e2d0d48d50d1687b93b97d5f7c6d5047406a5e688b352209bcb9f8227dde385d566332ecc0eabfa9cf7822fdf209f70024a57b1aa000c55b881f8111b2dcde494a5f485e5bca4bd88a2763aed1ca2b2fa8f0540678cd1e0f3ad80892",
    );
}

#[test]
fn brainpoolp512t1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &BRAINPOOLP512T1,
        1,
        "04640ece5c12788717b9c1ba06cbc2a6feba85842458c56dde9db1758d39c0313d82ba51735cdb3ea499aa77a7d6943a64f7a3f25fe26f06b51baa2696fa9035da5b534bd595f5af0fa2c892376c84ace1bb4e3019b71634c01131159cae03cee9d9932184beef216bd71df2dadf86a627306ecff96dbb8bace198b61e00f8b332",
    );
}

#[test]
fn gostr3410_2001_cryptopro_a_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &GOSTR3410_2001_CRYPTOPRO_A,
        1,
        "0400000000000000000000000000000000000000000000000000000000000000018d91e471e0989cda27df505a453f2b7635294f2ddf23e3b122acc99c9e9f1e14",
    );
}

#[test]
fn gostr3410_2001_cryptopro_b_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &GOSTR3410_2001_CRYPTOPRO_B,
        1,
        "0400000000000000000000000000000000000000000000000000000000000000013fa8124359f96680b83d1c3eb2c070e5c545c9858d03ecfb744bf8d717717efc",
    );
}

#[test]
fn gostr3410_2001_cryptopro_c_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &GOSTR3410_2001_CRYPTOPRO_C,
        1,
        "04000000000000000000000000000000000000000000000000000000000000000041ece55743711a8c3cbf3783cd08c0ee4d4dc440d4641a8f366e550dfdb3bb67",
    );
}

#[test]
fn tc26_gost_3410_12_256_paramseta_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &TC26_GOST_3410_12_256_PARAMSETA,
        4,
        "0491e38443a5e82c0d880923425712b2bb658b9196932e02c78b2582fe742daa2832879423ab1a0375895786c4bb46e9565fde0b5344766740af268adb32322e5c",
    );
}

#[test]
fn tc26_gost_3410_12_512_paramseta_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &TC26_GOST_3410_12_512_PARAMSETA,
        1,
        "04000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000037503cfe87a836ae3a61b8816e25450e6ce5e1c93acf1abc1778064fdcbefa921df1626be4fd036e93d75e6a50e3a41e98028fe5fc235f5b889a589cb5215f2a4",
    );
}

#[test]
fn tc26_gost_3410_12_512_paramsetb_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &TC26_GOST_3410_12_512_PARAMSETB,
        1,
        "04000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000021a8f7eda389b094c2c071e3647a8940f3c123b697578c213be6dd9e6c8ec7335dcb228fd1edf4a39152cbcaaf8c0398828041055f94ceeec7e21340780fe41bd",
    );
}

#[test]
fn tc26_gost_3410_12_512_paramsetc_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &TC26_GOST_3410_12_512_PARAMSETC,
        4,
        "04e2e31edfc23de7bdebe241ce593ef5de2295b7a9cbaef021d385f7074cea043aa27272a7ae602bf2a7b9033db9ed3610c6fb85487eae97aac5bc7928c1950148f5ce40d95b5eb899abbccff5911cb8577939804d6527378b8c108c3d2090ff9be18e2d33e3021ed2ef32d85822423b6304f726aa854bae07d0396e9a9addc40f",
    );
}

#[test]
fn prime192v2_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &PRIME192V2,
        1,
        "03eea2bae7e1497842f2de7769cfe9c989c072ad696f48034a",
    );
}

#[test]
fn prime192v3_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &PRIME192V3,
        1,
        "027d29778100c65a1da1783716588dce2b8b4aee8e228f1896",
    );
}

#[test]
fn prime239v1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &PRIME239V1,
        1,
        "020ffa963cdca8816ccc33b8642bedf905c3d358573d3f27fbbd3b3cb9aaaf",
    );
}

#[test]
fn prime239v2_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &PRIME239V2,
        1,
        "0238af09d98727705120c921bb5e9e26296a3cdcf2f35757a0eafd87b830e7",
    );
}

#[test]
fn prime239v3_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &PRIME239V3,
        1,
        "036768ae8e18bb92cfcf005c949aa2c6d94853d0e660bbf854b1c9505fe95a",
    );
}

#[test]
fn frp256v1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &FRP256V1,
        1,
        "04b6b3d4c356c139eb31183d4749d423958c27d2dcaf98b70164c97a2dd98f5cff6142e0f7c8b204911f9271f0f3ecef8c2701c307e8e4c9e183115a1554062cfb",
    );
}

#[test]
fn wapip192v1_holds_together_and_decodes_bouncy_castles_generator() {
    check(
        &WAPIP192V1,
        1,
        "044ad5f7048de709ad51236de65e4d4b482c836dc6e410664002bb3a02d4aaadacae24817a4ca3a1b014b5270432db27d2",
    );
}

#[test]
fn each_alias_is_the_curve_it_names() {
    let base_point = |named: &FpNamedCurve<32>| {
        let field = named.field();
        let curve = named.ct_curve(&field);
        named.ct_generator(&curve).encode(false)
    };
    for (alias, named) in [
        (GOSTR3410_2001_CRYPTOPRO_XCHA, GOSTR3410_2001_CRYPTOPRO_A),
        (GOSTR3410_2001_CRYPTOPRO_XCHB, GOSTR3410_2001_CRYPTOPRO_C),
        (TC26_GOST_3410_12_256_PARAMSETB, GOSTR3410_2001_CRYPTOPRO_A),
        (TC26_GOST_3410_12_256_PARAMSETC, GOSTR3410_2001_CRYPTOPRO_B),
        (TC26_GOST_3410_12_256_PARAMSETD, GOSTR3410_2001_CRYPTOPRO_C),
    ] {
        assert_eq!(base_point(&alias), base_point(&named));
    }
    let field = WAPI192V1.field();
    let curve = WAPI192V1.ct_curve(&field);
    let field_p = WAPIP192V1.field();
    let curve_p = WAPIP192V1.ct_curve(&field_p);
    assert_eq!(
        WAPI192V1.ct_generator(&curve).encode(false),
        WAPIP192V1.ct_generator(&curve_p).encode(false)
    );
}

// RFC 7836, example 7: the keys, little-endian there, on 512 paramSetA.
fn little_endian(hex: &str) -> PaddedBigUint {
    let mut bytes = bytes(hex);
    bytes.reverse();
    <PaddedBigUint as tc_bigint::ArrayEncoding<u8>>::from_be(&bytes).unwrap()
}

#[test]
fn the_keys_of_rfc_7836_example_7_lie_on_512_param_set_a() {
    use tc_ec_core::PrimeFieldElement;

    let named = TC26_GOST_3410_12_512_PARAMSETA;
    let field = named.field();
    let curve = named.ct_curve(&field);
    let g = named.ct_generator(&curve);
    for (d, x, y) in [
        (
            "c990ecd972fce84ec4db022778f50fcac726f46708384b8d458304962d7147f8c2db41cef22c90b102f2968404f9b9be6d47c79692d81826b32b8daca43cb667",
            "aab0eda4abff21208d18799fb9a8556654ba783070eba10cb9abb253ec56dcf5d3ccba6192e464e6e5bcb6dea137792f2431f6c897eb1b3c0cc14327b1adc0a7",
            "914613a3074e363aedb204d38d3563971bd8758e878c9db11403721b48002d38461f92472d40ea92f9958c0ffa4c93756401b97f89fdbe0b5e46e4a4631cdb5a",
        ),
        (
            "48c859f7b6f11585887cc05ec6ef1390cfea739b1a18c0d4662293ef63b79e3b8014070b44918590b4b996acfea4edfbbbcccc8c06edd8bf5bda92a51392d0db",
            "192fe183b9713a077253c72c8735de2ea42a3dbc66ea317838b65fa32523cd5efca974eda7c863f4954d1147f1f2b25c395fce1c129175e876d132e94ed5a651",
            "04883b414c9b592ec4dc84826f07d0b6d9006dda176ce48c391e3f97d102e03bb598bf132a228a45f7201aba08fc524a2d77e43a362ab022ad4028f75bde3b79",
        ),
    ] {
        let (qx, qy) = g.multiply(&little_endian(d)).affine().unwrap();
        assert_eq!(qx.to_big_uint(), little_endian(x));
        assert_eq!(qy.to_big_uint(), little_endian(y));
    }
}
