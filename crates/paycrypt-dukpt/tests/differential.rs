//! Differential tests against the moov-io/dukpt Go reference; run with `cargo test -- --ignored`.

use paycrypt_dukpt::codec::to_hex_upper;
use paycrypt_dukpt::tdes::{derive_ipek, derive_transaction_key};
use paycrypt_dukpt::types::{Bdk, TdesKsn};
use std::process::Command;

const BDK_HEX: &str = "0123456789ABCDEFFEDCBA9876543210";

// REF_MOOV_BIN runs a prebuilt shim on machines without Go proxy access.
fn moov(mode: &str, a: &str, b: &str) -> String {
    let out = match std::env::var("REF_MOOV_BIN") {
        Ok(bin) => Command::new(bin).args([mode, a, b]).output(),
        Err(_) => Command::new("go")
            .args([
                "run",
                concat!(env!("CARGO_MANIFEST_DIR"), "/../../scripts/ref_moov.go"),
                mode,
                a,
                b,
            ])
            .output(),
    }
    .expect("run moov-io reference shim");
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

#[test]
#[ignore = "requires the moov-io reference (REF_MOOV_BIN or go); differential CI job"]
fn tdes_ipek_matches_moov() {
    let bdk = Bdk::new(hex_literal::hex!("0123456789ABCDEFFEDCBA9876543210"));
    for ksn_hex in [
        "FFFF9876543210E00001",
        "FFFF9876543210E0000A",
        "FFFF9876543210E0001F",
    ] {
        let ksn = TdesKsn::from_hex(ksn_hex).unwrap();
        let ours = to_hex_upper(derive_ipek(&bdk, &ksn).as_bytes());
        assert_eq!(
            ours,
            moov("tdes-ipek", BDK_HEX, ksn_hex),
            "IPEK mismatch vs moov"
        );
    }
}

#[test]
#[ignore = "requires the moov-io reference (REF_MOOV_BIN or go); differential CI job"]
fn tdes_transaction_key_matches_moov() {
    let bdk = Bdk::new(hex_literal::hex!("0123456789ABCDEFFEDCBA9876543210"));
    // 0x1F sets five bits, exercising the full ladder.
    for ksn_hex in [
        "FFFF9876543210E00001",
        "FFFF9876543210E00003",
        "FFFF9876543210E0000A",
        "FFFF9876543210E0001F",
    ] {
        let ksn = TdesKsn::from_hex(ksn_hex).unwrap();
        let ipek = derive_ipek(&bdk, &ksn);
        let ours = to_hex_upper(derive_transaction_key(&ipek, &ksn).as_bytes());
        assert_eq!(
            ours,
            moov("tdes-txn", BDK_HEX, ksn_hex),
            "txn key mismatch vs moov"
        );
    }
}
