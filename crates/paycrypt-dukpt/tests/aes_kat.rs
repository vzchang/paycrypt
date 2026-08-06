//! Known-answer tests for AES DUKPT (X9.24-3).
//!
//! Source: ASC X9.24-3 test-vector PDF, cross-checked with moov-io/dukpt.

use paycrypt_dukpt::aes::*;
use paycrypt_dukpt::codec::to_hex_upper;

const BDK_128: [u8; 16] = hex_literal::hex!("FEDCBA9876543210F1F1F1F1F1F1F1F1");
const IK_ID: [u8; 8] = hex_literal::hex!("1234567890123456");

fn ksn_for(counter: u32) -> AesKsn {
    let mut b = [0u8; 12];
    b[..8].copy_from_slice(&IK_ID);
    b[8..].copy_from_slice(&counter.to_be_bytes());
    AesKsn::from_bytes(b)
}

#[test]
fn aes_initial_key() {
    let ik = derive_initial_key(&BDK_128, &IK_ID, KeyType::Aes128);
    assert_eq!(
        ik.as_bytes(),
        &hex_literal::hex!("1273671EA26AC29AFA4D1084127652A1")[..]
    );
}

#[test]
fn aes_pin_working_keys() {
    let ik = derive_initial_key(&BDK_128, &IK_ID, KeyType::Aes128);
    // 8675309 (0x845FED) sets many bits, so it walks the full intermediate-key ladder.
    for (counter, expected) in [
        (1u32, "AF8CB133A78F8DC2D1359F18527593FB"),
        (8675309u32, "D1DDA386AA4A556AF0119FDCB5D132C6"),
    ] {
        let ksn = ksn_for(counter);
        let wk = derive_working_key(ik.as_bytes(), &ksn, KeyUsage::PinEncryption, KeyType::Aes128);
        assert_eq!(
            to_hex_upper(wk.as_bytes()),
            expected,
            "PIN working key mismatch for counter {counter}"
        );
    }
}
