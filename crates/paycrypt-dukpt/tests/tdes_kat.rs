//! Known-answer tests for TDES DUKPT.
//!
//! Source: moov-io/dukpt (pkg/des tests) and the sgbj/Dukpt.NET README.

use paycrypt_dukpt::tdes::derive_ipek;
use paycrypt_dukpt::types::{Bdk, TdesKsn};

#[test]
fn tdes_ipek_canonical() {
    let bdk = Bdk::new(hex_literal::hex!("0123456789ABCDEFFEDCBA9876543210"));
    let ksn = TdesKsn::from_hex("FFFF9876543210E00008").unwrap();
    let ipek = derive_ipek(&bdk, &ksn);
    assert_eq!(
        ipek.as_bytes(),
        &hex_literal::hex!("6AC292FAA1315B4D858AB3A3D7D5933A")
    );
}

// Source: moov-io/dukpt (X9.24-1 Annex A.4). Raw ladder keys, before any variant mask.
#[test]
fn tdes_transaction_keys() {
    use paycrypt_dukpt::tdes::derive_transaction_key;
    let bdk = Bdk::new(hex_literal::hex!("0123456789ABCDEFFEDCBA9876543210"));
    for (ksn_hex, expected) in [
        ("FFFF9876543210E00001", "042666B49184CFA368DE9628D0397BC9"),
        ("FFFF9876543210E00002", "C46551CEF9FD24B0AA9AD834130D3BC7"),
        ("FFFF9876543210E00003", "0DF3D9422ACA56E547676D07AD6BADFA"),
    ] {
        let ksn = TdesKsn::from_hex(ksn_hex).unwrap();
        let ipek = derive_ipek(&bdk, &ksn);
        let tk = derive_transaction_key(&ipek, &ksn);
        assert_eq!(
            paycrypt_dukpt::codec::to_hex_upper(tk.as_bytes()),
            expected,
            "transaction key mismatch for KSN {ksn_hex}"
        );
    }
}

proptest::proptest! {
    #[test]
    fn prop_derive_deterministic(c in 1u32..4096) {
        use paycrypt_dukpt::tdes::derive_transaction_key;
        let bdk = Bdk::new(hex_literal::hex!("0123456789ABCDEFFEDCBA9876543210"));
        let ksn = TdesKsn::from_hex(&std::format!("FFFF9876543210E{c:05X}")).unwrap();
        let ipek = derive_ipek(&bdk, &ksn);
        let k1 = derive_transaction_key(&ipek, &ksn);
        let k2 = derive_transaction_key(&ipek, &ksn);
        proptest::prop_assert_eq!(k1.as_bytes(), k2.as_bytes());
    }
}
