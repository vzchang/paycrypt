//! The last step of each instrumented derivation equals the one-shot output.

use paycrypt_dukpt::tdes::{derive_ipek, derive_transaction_key, derive_transaction_key_steps};
use paycrypt_dukpt::types::{Bdk, TdesKsn};

#[test]
fn tdes_steps_final_equals_oneshot() {
    let bdk = Bdk::new(hex_literal::hex!("0123456789ABCDEFFEDCBA9876543210"));
    for ksn_hex in [
        "FFFF9876543210E00001",
        "FFFF9876543210E00003",
        "FFFF9876543210E0000A",
    ] {
        let ksn = TdesKsn::from_hex(ksn_hex).unwrap();
        let ipek = derive_ipek(&bdk, &ksn);
        let steps = derive_transaction_key_steps(&ipek, &ksn);
        let oneshot = derive_transaction_key(&ipek, &ksn);
        assert_eq!(
            steps.last().unwrap().bytes.as_slice(),
            oneshot.as_bytes(),
            "steps final != one-shot for KSN {ksn_hex}"
        );
        assert!(steps.len() >= 2, "expected IPEK + final for KSN {ksn_hex}");
    }
}
