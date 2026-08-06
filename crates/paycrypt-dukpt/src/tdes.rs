//! TDES DUKPT (ANSI X9.24-1) key derivation.

use crate::types::{Bdk, InitialKey, TdesKsn};
use cipher::{BlockCipherEncrypt, KeyInit};
use des::TdesEde2;

/// DUKPT key mask (ANSI X9.24-1); not a variant mask.
const KEY_MASK: [u8; 16] = [
    0xC0, 0xC0, 0xC0, 0xC0, 0x00, 0x00, 0x00, 0x00, 0xC0, 0xC0, 0xC0, 0xC0, 0x00, 0x00, 0x00, 0x00,
];

fn tdes_encrypt_block(key: &[u8; 16], block: &[u8; 8]) -> [u8; 8] {
    let cipher = TdesEde2::new_from_slice(key).expect("16-byte key is valid for TdesEde2");
    let mut buf = *block;
    cipher.encrypt_block((&mut buf).into());
    buf
}

fn xor16(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = a[i] ^ b[i];
    }
    out
}

/// Derive the Initial PIN Encryption Key (IPEK) from a BDK and a KSN.
pub fn derive_ipek(bdk: &Bdk, ksn: &TdesKsn) -> InitialKey {
    let d = ksn.initial_key_id();
    let bdk_bytes = bdk.as_bytes();

    let left = tdes_encrypt_block(bdk_bytes, &d);
    let masked = xor16(bdk_bytes, &KEY_MASK);
    let right = tdes_encrypt_block(&masked, &d);

    let mut ipek = [0u8; 16];
    ipek[..8].copy_from_slice(&left);
    ipek[8..].copy_from_slice(&right);
    InitialKey::new(ipek)
}
