use super::FpNamedCurve;
use super::hex::hex;

/// GostR3410-2001-CryptoPro-A of RFC 4357.
pub const GOSTR3410_2001_CRYPTOPRO_A: FpNamedCurve<32> = FpNamedCurve {
    p: hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd97"),
    a: hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd94"),
    b: hex("00000000000000000000000000000000000000000000000000000000000000a6"),
    order: hex("ffffffffffffffffffffffffffffffff6c611070995ad10045841b09b761b893"),
    cofactor: 1,
    gx: hex("0000000000000000000000000000000000000000000000000000000000000001"),
    gy: hex("8d91e471e0989cda27df505a453f2b7635294f2ddf23e3b122acc99c9e9f1e14"),
    glv: None,
};

/// GostR3410-2001-CryptoPro-B of RFC 4357.
pub const GOSTR3410_2001_CRYPTOPRO_B: FpNamedCurve<32> = FpNamedCurve {
    p: hex("8000000000000000000000000000000000000000000000000000000000000c99"),
    a: hex("8000000000000000000000000000000000000000000000000000000000000c96"),
    b: hex("3e1af419a269a5f866a7d3c25c3df80ae979259373ff2b182f49d4ce7e1bbc8b"),
    order: hex("800000000000000000000000000000015f700cfff1a624e5e497161bcc8a198f"),
    cofactor: 1,
    gx: hex("0000000000000000000000000000000000000000000000000000000000000001"),
    gy: hex("3fa8124359f96680b83d1c3eb2c070e5c545c9858d03ecfb744bf8d717717efc"),
    glv: None,
};

/// GostR3410-2001-CryptoPro-C of RFC 4357.
pub const GOSTR3410_2001_CRYPTOPRO_C: FpNamedCurve<32> = FpNamedCurve {
    p: hex("9b9f605f5a858107ab1ec85e6b41c8aacf846e86789051d37998f7b9022d759b"),
    a: hex("9b9f605f5a858107ab1ec85e6b41c8aacf846e86789051d37998f7b9022d7598"),
    b: hex("000000000000000000000000000000000000000000000000000000000000805a"),
    order: hex("9b9f605f5a858107ab1ec85e6b41c8aa582ca3511eddfb74f02f3a6598980bb9"),
    cofactor: 1,
    gx: hex("0000000000000000000000000000000000000000000000000000000000000000"),
    gy: hex("41ece55743711a8c3cbf3783cd08c0ee4d4dc440d4641a8f366e550dfdb3bb67"),
    glv: None,
};

/// GostR3410-2001-CryptoPro-XchA of RFC 4357, the parameters of CryptoPro-A.
pub const GOSTR3410_2001_CRYPTOPRO_XCHA: FpNamedCurve<32> = GOSTR3410_2001_CRYPTOPRO_A;

/// GostR3410-2001-CryptoPro-XchB of RFC 4357, the parameters of CryptoPro-C.
pub const GOSTR3410_2001_CRYPTOPRO_XCHB: FpNamedCurve<32> = GOSTR3410_2001_CRYPTOPRO_C;

/// id-tc26-gost-3410-12-256-paramSetA of RFC 7836, whose cofactor is 4.
pub const TC26_GOST_3410_12_256_PARAMSETA: FpNamedCurve<32> = FpNamedCurve {
    p: hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd97"),
    a: hex("c2173f1513981673af4892c23035a27ce25e2013bf95aa33b22c656f277e7335"),
    b: hex("295f9bae7428ed9ccc20e7c359a9d41a22fccd9108e17bf7ba9337a6f8ae9513"),
    order: hex("400000000000000000000000000000000fd8cddfc87b6635c115af556c360c67"),
    cofactor: 4,
    gx: hex("91e38443a5e82c0d880923425712b2bb658b9196932e02c78b2582fe742daa28"),
    gy: hex("32879423ab1a0375895786c4bb46e9565fde0b5344766740af268adb32322e5c"),
    glv: None,
};

/// id-tc26-gost-3410-12-256-paramSetB of RFC 7836, the parameters of CryptoPro-A.
pub const TC26_GOST_3410_12_256_PARAMSETB: FpNamedCurve<32> = GOSTR3410_2001_CRYPTOPRO_A;

/// id-tc26-gost-3410-12-256-paramSetC of RFC 7836, the parameters of CryptoPro-B.
pub const TC26_GOST_3410_12_256_PARAMSETC: FpNamedCurve<32> = GOSTR3410_2001_CRYPTOPRO_B;

/// id-tc26-gost-3410-12-256-paramSetD of RFC 7836, the parameters of CryptoPro-C.
pub const TC26_GOST_3410_12_256_PARAMSETD: FpNamedCurve<32> = GOSTR3410_2001_CRYPTOPRO_C;

/// id-tc26-gost-3410-12-512-paramSetA of RFC 7836.
pub const TC26_GOST_3410_12_512_PARAMSETA: FpNamedCurve<64> = FpNamedCurve {
    p: hex(
        "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdc7",
    ),
    a: hex(
        "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdc4",
    ),
    b: hex(
        "e8c2505dedfc86ddc1bd0b2b6667f1da34b82574761cb0e879bd081cfd0b6265ee3cb090f30d27614cb4574010da90dd862ef9d4ebee4761503190785a71c760",
    ),
    order: hex(
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27e69532f48d89116ff22b8d4e0560609b4b38abfad2b85dcacdb1411f10b275",
    ),
    cofactor: 1,
    gx: hex(
        "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003",
    ),
    gy: hex(
        "7503cfe87a836ae3a61b8816e25450e6ce5e1c93acf1abc1778064fdcbefa921df1626be4fd036e93d75e6a50e3a41e98028fe5fc235f5b889a589cb5215f2a4",
    ),
    glv: None,
};

/// id-tc26-gost-3410-12-512-paramSetB of RFC 7836.
pub const TC26_GOST_3410_12_512_PARAMSETB: FpNamedCurve<64> = FpNamedCurve {
    p: hex(
        "8000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006f",
    ),
    a: hex(
        "8000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006c",
    ),
    b: hex(
        "687d1b459dc841457e3e06cf6f5e2517b97c7d614af138bcbf85dc806c4b289f3e965d2db1416d217f8b276fad1ab69c50f78bee1fa3106efb8ccbc7c5140116",
    ),
    order: hex(
        "800000000000000000000000000000000000000000000000000000000000000149a1ec142565a545acfdb77bd9d40cfa8b996712101bea0ec6346c54374f25bd",
    ),
    cofactor: 1,
    gx: hex(
        "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002",
    ),
    gy: hex(
        "1a8f7eda389b094c2c071e3647a8940f3c123b697578c213be6dd9e6c8ec7335dcb228fd1edf4a39152cbcaaf8c0398828041055f94ceeec7e21340780fe41bd",
    ),
    glv: None,
};

/// id-tc26-gost-3410-12-512-paramSetC of RFC 7836, whose cofactor is 4.
pub const TC26_GOST_3410_12_512_PARAMSETC: FpNamedCurve<64> = FpNamedCurve {
    p: hex(
        "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdc7",
    ),
    a: hex(
        "dc9203e514a721875485a529d2c722fb187bc8980eb866644de41c68e143064546e861c0e2c9edd92ade71f46fcf50ff2ad97f951fda9f2a2eb6546f39689bd3",
    ),
    b: hex(
        "b4c4ee28cebc6c2c8ac12952cf37f16ac7efb6a9f69f4b57ffda2e4f0de5ade038cbc2fff719d2c18de0284b8bfef3b52b8cc7a5f5bf0a3c8d2319a5312557e1",
    ),
    order: hex(
        "3fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc98cdba46506ab004c33a9ff5147502cc8eda9e7a769a12694623cef47f023ed",
    ),
    cofactor: 4,
    gx: hex(
        "e2e31edfc23de7bdebe241ce593ef5de2295b7a9cbaef021d385f7074cea043aa27272a7ae602bf2a7b9033db9ed3610c6fb85487eae97aac5bc7928c1950148",
    ),
    gy: hex(
        "f5ce40d95b5eb899abbccff5911cb8577939804d6527378b8c108c3d2090ff9be18e2d33e3021ed2ef32d85822423b6304f726aa854bae07d0396e9a9addc40f",
    ),
    glv: None,
};
