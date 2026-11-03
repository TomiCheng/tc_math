use super::FpNamedCurve;
use super::hex::hex;

/// wapip192v1 of the Chinese WAPI standard, GB 15629.11.
pub const WAPIP192V1: FpNamedCurve<24> = FpNamedCurve {
    p: hex("bdb6f4fe3e8b1d9e0da8c0d46f4c318cefe4afe3b6b8551f"),
    a: hex("bb8e5e8fbc115e139fe6a814fe48aaa6f0ada1aa5df91985"),
    b: hex("1854bebdc31b21b7aefc80ab0ecd10d5b1b3308e6dbf11c1"),
    order: hex("bdb6f4fe3e8b1d9e0da8c0d40fc962195dfae76f56564677"),
    cofactor: 1,
    gx: hex("4ad5f7048de709ad51236de65e4d4b482c836dc6e4106640"),
    gy: hex("02bb3a02d4aaadacae24817a4ca3a1b014b5270432db27d2"),
    glv: None,
};

/// wapi192v1, the parameters of wapip192v1 under another OID.
pub const WAPI192V1: FpNamedCurve<24> = WAPIP192V1;
