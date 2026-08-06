//! Differential tests against the moov-io/dukpt Go reference; run with `cargo test -- --ignored`.

use paycrypt_dukpt::codec::to_hex_upper;
use paycrypt_dukpt::tdes::derive_ipek;
use paycrypt_dukpt::types::{Bdk, TdesKsn};
use std::process::Command;

fn moov(mode: &str, a: &str, b: &str) -> String {
    let out = Command::new("go")
        .args(["run", "scripts/ref_moov.go", mode, a, b])
        .output()
        .expect("run ref_moov.go");
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

#[test]
#[ignore = "requires go + moov-io/dukpt; run via the differential CI job"]
fn tdes_ipek_matches_moov() {
    let bdk_hex = "0123456789ABCDEFFEDCBA9876543210";
    for ksn_hex in ["FFFF9876543210E00001", "FFFF9876543210E0000A"] {
        let bdk = Bdk::new(hex_literal::hex!("0123456789ABCDEFFEDCBA9876543210"));
        let ksn = TdesKsn::from_hex(ksn_hex).unwrap();
        let ours = to_hex_upper(derive_ipek(&bdk, &ksn).as_bytes());
        assert_eq!(ours, moov("tdes-ipek", bdk_hex, ksn_hex), "IPEK mismatch vs moov");
    }
}
