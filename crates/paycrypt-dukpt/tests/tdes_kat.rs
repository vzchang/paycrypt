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
