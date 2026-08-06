//! Known-answer tests for ISO 9564 clear PIN block formats 0-3.
//!
//! Source: knovichikhin/psec `tests/test_pinblock.py` and EFTLab BP-Tools examples.

use hex_literal::hex;
use paycrypt_pinblock::clear::{ClearPinBlockCodec, Iso0, Iso1, Iso2, Iso3};
use paycrypt_pinblock::types::{ClearPinBlock, FixedRng, Pan, Pin};

// Format 0

#[test]
fn iso0_pin_1234_pan_5555() {
    // psec
    let pin = Pin::new("1234").unwrap();
    let pan = Pan::new("5555555551234567").unwrap();
    let mut rng = FixedRng::new(&[]);
    let block = Iso0.format(&pin, Some(&pan), &mut rng).unwrap();
    assert_eq!(block.as_bytes(), &hex!("041261AAAAEDCBA9"));
    let recovered = Iso0.parse(&block, Some(&pan)).unwrap();
    assert_eq!(recovered, pin);
}

#[test]
fn iso0_pin_12digits() {
    // psec
    let pin = Pin::new("123456789012").unwrap();
    let pan = Pan::new("5555555551234567").unwrap();
    let mut rng = FixedRng::new(&[]);
    let block = Iso0.format(&pin, Some(&pan), &mut rng).unwrap();
    assert_eq!(block.as_bytes(), &hex!("0C1261032D8226A9"));
    assert_eq!(Iso0.parse(&block, Some(&pan)).unwrap(), pin);
}

#[test]
fn iso0_pin_1234_pan_alt() {
    // psec, EFTLab
    let pin = Pin::new("1234").unwrap();
    let pan = Pan::new("5544332211009966").unwrap();
    let mut rng = FixedRng::new(&[]);
    let block = Iso0.format(&pin, Some(&pan), &mut rng).unwrap();
    assert_eq!(block.as_bytes(), &hex!("041277CDDEEFF669"));
    assert_eq!(Iso0.parse(&block, Some(&pan)).unwrap(), pin);
}

#[test]
fn iso0_requires_pan() {
    let pin = Pin::new("1234").unwrap();
    let mut rng = FixedRng::new(&[]);
    assert!(Iso0.format(&pin, None, &mut rng).is_err());
}

// Format 2

#[test]
fn iso2_vectors() {
    let cases: &[(&str, [u8; 8])] = &[
        ("1234", hex!("241234FFFFFFFFFF")),
        ("123456789", hex!("29123456789FFFFF")),
        ("1234567890", hex!("2A1234567890FFFF")),
        ("123456789012", hex!("2C123456789012FF")),
    ];
    let mut rng = FixedRng::new(&[]);
    for (pin_str, expected) in cases {
        let pin = Pin::new(pin_str).unwrap();
        let block = Iso2.format(&pin, None, &mut rng).unwrap();
        assert_eq!(block.as_bytes(), expected, "encode {pin_str}");
        assert_eq!(Iso2.parse(&block, None).unwrap(), pin, "decode {pin_str}");
    }
}

// Format 3

#[test]
fn iso3_decode_fixed_block() {
    // psec, decode direction
    let block = ClearPinBlock::from_bytes(hex!("341261AAAAEDCBA9"));
    let pan = Pan::new("5555555551234567").unwrap();
    let pin = Iso3.parse(&block, Some(&pan)).unwrap();
    assert_eq!(pin, Pin::new("1234").unwrap());
}

#[test]
fn iso3_roundtrip_fixedrng() {
    let pin = Pin::new("1234").unwrap();
    let pan = Pan::new("5555555551234567").unwrap();
    let mut rng = FixedRng::new(&hex!("AABBCCDD"));
    let block = Iso3.format(&pin, Some(&pan), &mut rng).unwrap();
    assert_eq!(Iso3.parse(&block, Some(&pan)).unwrap(), pin);
}

// Format 1: round-trip only; there's no published vector.

#[test]
fn iso1_roundtrip_fixedrng() {
    let pin = Pin::new("987654").unwrap();
    let mut rng = FixedRng::new(&hex!("0102030405060708"));
    let block = Iso1.format(&pin, None, &mut rng).unwrap();
    assert_eq!(block.as_bytes()[0] >> 4, 1, "control nibble is 1");
    assert_eq!(Iso1.parse(&block, None).unwrap(), pin);
}

// Errors

#[test]
fn rejects_bad_pin_length() {
    assert!(Pin::new("123").is_err());
    assert!(Pin::new("1234567890123").is_err());
}

#[test]
fn rejects_non_digit() {
    assert!(Pin::new("12A4").is_err());
    assert!(Pan::new("12A4").is_err());
}
