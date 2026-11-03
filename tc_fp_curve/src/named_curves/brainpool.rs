use super::FpNamedCurve;
use super::hex::hex;

/// RFC 5639 brainpoolP160r1.
pub const BRAINPOOLP160R1: FpNamedCurve<20> = FpNamedCurve {
    p: hex("e95e4a5f737059dc60dfc7ad95b3d8139515620f"),
    a: hex("340e7be2a280eb74e2be61bada745d97e8f7c300"),
    b: hex("1e589a8595423412134faa2dbdec95c8d8675e58"),
    order: hex("e95e4a5f737059dc60df5991d45029409e60fc09"),
    cofactor: 1,
    gx: hex("bed5af16ea3f6a4f62938c4631eb5af7bdbcdbc3"),
    gy: hex("1667cb477a1a8ec338f94741669c976316da6321"),
    glv: None,
};

/// RFC 5639 brainpoolP160t1, isomorphic to brainpoolP160r1, with a = -3.
pub const BRAINPOOLP160T1: FpNamedCurve<20> = FpNamedCurve {
    p: hex("e95e4a5f737059dc60dfc7ad95b3d8139515620f"),
    a: hex("e95e4a5f737059dc60dfc7ad95b3d8139515620c"),
    b: hex("7a556b6dae535b7b51ed2c4d7daa7a0b5c55f380"),
    order: hex("e95e4a5f737059dc60df5991d45029409e60fc09"),
    cofactor: 1,
    gx: hex("b199b13b9b34efc1397e64baeb05acc265ff2378"),
    gy: hex("add6718b7c7c1961f0991b842443772152c9e0ad"),
    glv: None,
};

/// RFC 5639 brainpoolP192r1.
pub const BRAINPOOLP192R1: FpNamedCurve<24> = FpNamedCurve {
    p: hex("c302f41d932a36cda7a3463093d18db78fce476de1a86297"),
    a: hex("6a91174076b1e0e19c39c031fe8685c1cae040e5c69a28ef"),
    b: hex("469a28ef7c28cca3dc721d044f4496bcca7ef4146fbf25c9"),
    order: hex("c302f41d932a36cda7a3462f9e9e916b5be8f1029ac4acc1"),
    cofactor: 1,
    gx: hex("c0a0647eaab6a48753b033c56cb0f0900a2f5c4853375fd6"),
    gy: hex("14b690866abd5bb88b5f4828c1490002e6773fa2fa299b8f"),
    glv: None,
};

/// RFC 5639 brainpoolP192t1, isomorphic to brainpoolP192r1, with a = -3.
pub const BRAINPOOLP192T1: FpNamedCurve<24> = FpNamedCurve {
    p: hex("c302f41d932a36cda7a3463093d18db78fce476de1a86297"),
    a: hex("c302f41d932a36cda7a3463093d18db78fce476de1a86294"),
    b: hex("13d56ffaec78681e68f9deb43b35bec2fb68542e27897b79"),
    order: hex("c302f41d932a36cda7a3462f9e9e916b5be8f1029ac4acc1"),
    cofactor: 1,
    gx: hex("3ae9e58c82f63c30282e1fe7bbf43fa72c446af6f4618129"),
    gy: hex("097e2c5667c2223a902ab5ca449d0084b7e5b3de7ccc01c9"),
    glv: None,
};

/// RFC 5639 brainpoolP224r1.
pub const BRAINPOOLP224R1: FpNamedCurve<28> = FpNamedCurve {
    p: hex("d7c134aa264366862a18302575d1d787b09f075797da89f57ec8c0ff"),
    a: hex("68a5e62ca9ce6c1c299803a6c1530b514e182ad8b0042a59cad29f43"),
    b: hex("2580f63ccfe44138870713b1a92369e33e2135d266dbb372386c400b"),
    order: hex("d7c134aa264366862a18302575d0fb98d116bc4b6ddebca3a5a7939f"),
    cofactor: 1,
    gx: hex("0d9029ad2c7e5cf4340823b2a87dc68c9e4ce3174c1e6efdee12c07d"),
    gy: hex("58aa56f772c0726f24c6b89e4ecdac24354b9e99caa3f6d3761402cd"),
    glv: None,
};

/// RFC 5639 brainpoolP224t1, isomorphic to brainpoolP224r1, with a = -3.
pub const BRAINPOOLP224T1: FpNamedCurve<28> = FpNamedCurve {
    p: hex("d7c134aa264366862a18302575d1d787b09f075797da89f57ec8c0ff"),
    a: hex("d7c134aa264366862a18302575d1d787b09f075797da89f57ec8c0fc"),
    b: hex("4b337d934104cd7bef271bf60ced1ed20da14c08b3bb64f18a60888d"),
    order: hex("d7c134aa264366862a18302575d0fb98d116bc4b6ddebca3a5a7939f"),
    cofactor: 1,
    gx: hex("6ab1e344ce25ff3896424e7ffe14762ecb49f8928ac0c76029b4d580"),
    gy: hex("0374e9f5143e568cd23f3f4d7c0d4b1e41c8cc0d1c6abd5f1a46db4c"),
    glv: None,
};

/// RFC 5639 brainpoolP256r1.
pub const BRAINPOOLP256R1: FpNamedCurve<32> = FpNamedCurve {
    p: hex("a9fb57dba1eea9bc3e660a909d838d726e3bf623d52620282013481d1f6e5377"),
    a: hex("7d5a0975fc2c3057eef67530417affe7fb8055c126dc5c6ce94a4b44f330b5d9"),
    b: hex("26dc5c6ce94a4b44f330b5d9bbd77cbf958416295cf7e1ce6bccdc18ff8c07b6"),
    order: hex("a9fb57dba1eea9bc3e660a909d838d718c397aa3b561a6f7901e0e82974856a7"),
    cofactor: 1,
    gx: hex("8bd2aeb9cb7e57cb2c4b482ffc81b7afb9de27e1e3bd23c23a4453bd9ace3262"),
    gy: hex("547ef835c3dac4fd97f8461a14611dc9c27745132ded8e545c1d54c72f046997"),
    glv: None,
};

/// RFC 5639 brainpoolP256t1, isomorphic to brainpoolP256r1, with a = -3.
pub const BRAINPOOLP256T1: FpNamedCurve<32> = FpNamedCurve {
    p: hex("a9fb57dba1eea9bc3e660a909d838d726e3bf623d52620282013481d1f6e5377"),
    a: hex("a9fb57dba1eea9bc3e660a909d838d726e3bf623d52620282013481d1f6e5374"),
    b: hex("662c61c430d84ea4fe66a7733d0b76b7bf93ebc4af2f49256ae58101fee92b04"),
    order: hex("a9fb57dba1eea9bc3e660a909d838d718c397aa3b561a6f7901e0e82974856a7"),
    cofactor: 1,
    gx: hex("a3e8eb3cc1cfe7b7732213b23a656149afa142c47aafbc2b79a191562e1305f4"),
    gy: hex("2d996c823439c56d7f7b22e14644417e69bcb6de39d027001dabe8f35b25c9be"),
    glv: None,
};

/// RFC 5639 brainpoolP320r1.
pub const BRAINPOOLP320R1: FpNamedCurve<40> = FpNamedCurve {
    p: hex("d35e472036bc4fb7e13c785ed201e065f98fcfa6f6f40def4f92b9ec7893ec28fcd412b1f1b32e27"),
    a: hex("3ee30b568fbab0f883ccebd46d3f3bb8a2a73513f5eb79da66190eb085ffa9f492f375a97d860eb4"),
    b: hex("520883949dfdbc42d3ad198640688a6fe13f41349554b49acc31dccd884539816f5eb4ac8fb1f1a6"),
    order: hex("d35e472036bc4fb7e13c785ed201e065f98fcfa5b68f12a32d482ec7ee8658e98691555b44c59311"),
    cofactor: 1,
    gx: hex("43bd7e9afb53d8b85289bcc48ee5bfe6f20137d10a087eb6e7871e2a10a599c710af8d0d39e20611"),
    gy: hex("14fdd05545ec1cc8ab4093247f77275e0743ffed117182eaa9c77877aaac6ac7d35245d1692e8ee1"),
    glv: None,
};

/// RFC 5639 brainpoolP320t1, isomorphic to brainpoolP320r1, with a = -3.
pub const BRAINPOOLP320T1: FpNamedCurve<40> = FpNamedCurve {
    p: hex("d35e472036bc4fb7e13c785ed201e065f98fcfa6f6f40def4f92b9ec7893ec28fcd412b1f1b32e27"),
    a: hex("d35e472036bc4fb7e13c785ed201e065f98fcfa6f6f40def4f92b9ec7893ec28fcd412b1f1b32e24"),
    b: hex("a7f561e038eb1ed560b3d147db782013064c19f27ed27c6780aaf77fb8a547ceb5b4fef422340353"),
    order: hex("d35e472036bc4fb7e13c785ed201e065f98fcfa5b68f12a32d482ec7ee8658e98691555b44c59311"),
    cofactor: 1,
    gx: hex("925be9fb01afc6fb4d3e7d4990010f813408ab106c4f09cb7ee07868cc136fff3357f624a21bed52"),
    gy: hex("63ba3a7a27483ebf6671dbef7abb30ebee084e58a0b077ad42a5a0989d1ee71b1b9bc0455fb0d2c3"),
    glv: None,
};

/// RFC 5639 brainpoolP384r1.
pub const BRAINPOOLP384R1: FpNamedCurve<48> = FpNamedCurve {
    p: hex(
        "8cb91e82a3386d280f5d6f7e50e641df152f7109ed5456b412b1da197fb71123acd3a729901d1a71874700133107ec53",
    ),
    a: hex(
        "7bc382c63d8c150c3c72080ace05afa0c2bea28e4fb22787139165efba91f90f8aa5814a503ad4eb04a8c7dd22ce2826",
    ),
    b: hex(
        "04a8c7dd22ce28268b39b55416f0447c2fb77de107dcd2a62e880ea53eeb62d57cb4390295dbc9943ab78696fa504c11",
    ),
    order: hex(
        "8cb91e82a3386d280f5d6f7e50e641df152f7109ed5456b31f166e6cac0425a7cf3ab6af6b7fc3103b883202e9046565",
    ),
    cofactor: 1,
    gx: hex(
        "1d1c64f068cf45ffa2a63a81b7c13f6b8847a3e77ef14fe3db7fcafe0cbd10e8e826e03436d646aaef87b2e247d4af1e",
    ),
    gy: hex(
        "8abe1d7520f9c2a45cb1eb8e95cfd55262b70b29feec5864e19c054ff99129280e4646217791811142820341263c5315",
    ),
    glv: None,
};

/// RFC 5639 brainpoolP384t1, isomorphic to brainpoolP384r1, with a = -3.
pub const BRAINPOOLP384T1: FpNamedCurve<48> = FpNamedCurve {
    p: hex(
        "8cb91e82a3386d280f5d6f7e50e641df152f7109ed5456b412b1da197fb71123acd3a729901d1a71874700133107ec53",
    ),
    a: hex(
        "8cb91e82a3386d280f5d6f7e50e641df152f7109ed5456b412b1da197fb71123acd3a729901d1a71874700133107ec50",
    ),
    b: hex(
        "7f519eada7bda81bd826dba647910f8c4b9346ed8ccdc64e4b1abd11756dce1d2074aa263b88805ced70355a33b471ee",
    ),
    order: hex(
        "8cb91e82a3386d280f5d6f7e50e641df152f7109ed5456b31f166e6cac0425a7cf3ab6af6b7fc3103b883202e9046565",
    ),
    cofactor: 1,
    gx: hex(
        "18de98b02db9a306f2afcd7235f72a819b80ab12ebd653172476fecd462aabffc4ff191b946a5f54d8d0aa2f418808cc",
    ),
    gy: hex(
        "25ab056962d30651a114afd2755ad336747f93475b7a1fca3b88f2b6a208ccfe469408584dc2b2912675bf5b9e582928",
    ),
    glv: None,
};

/// RFC 5639 brainpoolP512r1.
pub const BRAINPOOLP512R1: FpNamedCurve<64> = FpNamedCurve {
    p: hex(
        "aadd9db8dbe9c48b3fd4e6ae33c9fc07cb308db3b3c9d20ed6639cca703308717d4d9b009bc66842aecda12ae6a380e62881ff2f2d82c68528aa6056583a48f3",
    ),
    a: hex(
        "7830a3318b603b89e2327145ac234cc594cbdd8d3df91610a83441caea9863bc2ded5d5aa8253aa10a2ef1c98b9ac8b57f1117a72bf2c7b9e7c1ac4d77fc94ca",
    ),
    b: hex(
        "3df91610a83441caea9863bc2ded5d5aa8253aa10a2ef1c98b9ac8b57f1117a72bf2c7b9e7c1ac4d77fc94cadc083e67984050b75ebae5dd2809bd638016f723",
    ),
    order: hex(
        "aadd9db8dbe9c48b3fd4e6ae33c9fc07cb308db3b3c9d20ed6639cca70330870553e5c414ca92619418661197fac10471db1d381085ddaddb58796829ca90069",
    ),
    cofactor: 1,
    gx: hex(
        "81aee4bdd82ed9645a21322e9c4c6a9385ed9f70b5d916c1b43b62eef4d0098eff3b1f78e2d0d48d50d1687b93b97d5f7c6d5047406a5e688b352209bcb9f822",
    ),
    gy: hex(
        "7dde385d566332ecc0eabfa9cf7822fdf209f70024a57b1aa000c55b881f8111b2dcde494a5f485e5bca4bd88a2763aed1ca2b2fa8f0540678cd1e0f3ad80892",
    ),
    glv: None,
};

/// RFC 5639 brainpoolP512t1, isomorphic to brainpoolP512r1, with a = -3.
pub const BRAINPOOLP512T1: FpNamedCurve<64> = FpNamedCurve {
    p: hex(
        "aadd9db8dbe9c48b3fd4e6ae33c9fc07cb308db3b3c9d20ed6639cca703308717d4d9b009bc66842aecda12ae6a380e62881ff2f2d82c68528aa6056583a48f3",
    ),
    a: hex(
        "aadd9db8dbe9c48b3fd4e6ae33c9fc07cb308db3b3c9d20ed6639cca703308717d4d9b009bc66842aecda12ae6a380e62881ff2f2d82c68528aa6056583a48f0",
    ),
    b: hex(
        "7cbbbcf9441cfab76e1890e46884eae321f70c0bcb4981527897504bec3e36a62bcdfa2304976540f6450085f2dae145c22553b465763689180ea2571867423e",
    ),
    order: hex(
        "aadd9db8dbe9c48b3fd4e6ae33c9fc07cb308db3b3c9d20ed6639cca70330870553e5c414ca92619418661197fac10471db1d381085ddaddb58796829ca90069",
    ),
    cofactor: 1,
    gx: hex(
        "640ece5c12788717b9c1ba06cbc2a6feba85842458c56dde9db1758d39c0313d82ba51735cdb3ea499aa77a7d6943a64f7a3f25fe26f06b51baa2696fa9035da",
    ),
    gy: hex(
        "5b534bd595f5af0fa2c892376c84ace1bb4e3019b71634c01131159cae03cee9d9932184beef216bd71df2dadf86a627306ecff96dbb8bace198b61e00f8b332",
    ),
    glv: None,
};
