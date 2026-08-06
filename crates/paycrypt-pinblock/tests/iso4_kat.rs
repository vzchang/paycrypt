//! Known-answer tests for the ISO 9564 format 4 codec.
//!
//! Sources: the ASC X9.24-3-2017 test-vector supplement and knovichikhin/psec.

use hex_literal::hex;
use paycrypt_pinblock::iso4::{EncipheringPinBlockCodec, EncryptedPinBlock, Iso4};
use paycrypt_pinblock::types::{FixedRng, Pan, Pin};

#[test]
fn iso4_encipher_x924_3() {
    // X9.24-3 supplement, format 4 worked example
    let key = hex!("AF8CB133A78F8DC2D1359F18527593FB");
    let pin = Pin::new("1234").unwrap();
    let pan = Pan::new("4111111111111111").unwrap();
    let mut rng = FixedRng::new(&hex!("2F69ADDE2E9E7ACE"));
    let block = Iso4.encipher(&key, &pin, &pan, &mut rng).unwrap();
    assert_eq!(block.as_bytes(), &hex!("A912150391AB65A67E52883D81CE2D15"));
    assert_eq!(Iso4.decipher(&key, &block, &pan).unwrap(), pin);
}

#[test]
fn iso4_decipher_psec() {
    // psec, decipher direction
    let key = hex!("00112233445566778899AABBCCDDEEFF");
    let block = EncryptedPinBlock::from_bytes(hex!("E4BE5B623AF7E006AC319E5B93544564"));
    let pan = Pan::new("1234567890123456").unwrap();
    assert_eq!(
        Iso4.decipher(&key, &block, &pan).unwrap(),
        Pin::new("1234").unwrap()
    );
}
