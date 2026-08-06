//! TDES DUKPT (ANSI X9.24-1) key derivation.

use crate::types::{Bdk, InitialKey, TdesKsn, TransactionKey};
use cipher::{BlockCipherEncrypt, KeyInit};
use des::{Des, TdesEde2};

const COUNTER_MASK: u32 = 0x1F_FFFF;

/// DUKPT variant masks (ANSI X9.24-1).
const PIN_MASK: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
];
const MAC_MASK: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00,
];
const DATA_MASK: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00,
];

/// Which working-key variant to derive from a transaction key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TdesVariant {
    /// PIN encryption key.
    Pin,
    /// MAC key.
    Mac,
    /// Data encryption key (XOR variant only, without the one-way re-encryption some devices add).
    Data,
}

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

fn des_encrypt_block(key: &[u8; 8], block: &[u8; 8]) -> [u8; 8] {
    let cipher = Des::new_from_slice(key).expect("8-byte DES key");
    let mut buf = *block;
    cipher.encrypt_block((&mut buf).into());
    buf
}

/// X9.24-1 `EncryptRegister`.
fn encrypt_register(key: &[u8; 16], reg: &[u8; 8]) -> [u8; 8] {
    let key_left: [u8; 8] = key[..8].try_into().unwrap();
    let key_right: [u8; 8] = key[8..].try_into().unwrap();
    let mut input = [0u8; 8];
    for i in 0..8 {
        input[i] = key_right[i] ^ reg[i];
    }
    let enc = des_encrypt_block(&key_left, &input);
    let mut out = [0u8; 8];
    for i in 0..8 {
        out[i] = enc[i] ^ key_right[i];
    }
    out
}

/// X9.24-1 `GenerateKey`.
fn generate_key(key: &[u8; 16], ksn_reg: &[u8; 8]) -> [u8; 16] {
    let masked = xor16(key, &KEY_MASK);
    let left = encrypt_register(&masked, ksn_reg);
    let right = encrypt_register(key, ksn_reg);
    let mut out = [0u8; 16];
    out[..8].copy_from_slice(&left);
    out[8..].copy_from_slice(&right);
    out
}

/// Derive the per-transaction key, one `GenerateKey` per set counter bit.
pub fn derive_transaction_key(ipek: &InitialKey, ksn: &TdesKsn) -> TransactionKey {
    let counter = ksn.transaction_counter();

    let ksn_bytes = ksn.as_bytes();
    let mut ksn_reg = [0u8; 8];
    ksn_reg.copy_from_slice(&ksn_bytes[2..10]);
    let tail = u32::from_be_bytes([ksn_reg[4], ksn_reg[5], ksn_reg[6], ksn_reg[7]]) & !COUNTER_MASK;
    ksn_reg[4..8].copy_from_slice(&tail.to_be_bytes());

    let mut cur_key = *ipek.as_bytes();
    let mut shift_reg: u32 = 0x10_0000; // bit 20 (MSB of the 21-bit counter)
    while shift_reg > 0 {
        if (shift_reg & counter) != 0 {
            let tail =
                u32::from_be_bytes([ksn_reg[4], ksn_reg[5], ksn_reg[6], ksn_reg[7]]) | shift_reg;
            ksn_reg[4..8].copy_from_slice(&tail.to_be_bytes());
            cur_key = generate_key(&cur_key, &ksn_reg);
        }
        shift_reg >>= 1;
    }
    TransactionKey::new(cur_key)
}

/// Step-instrumented form of [`derive_transaction_key`] for the explainer.
pub fn derive_transaction_key_steps(
    ipek: &InitialKey,
    ksn: &TdesKsn,
) -> alloc::vec::Vec<crate::steps::Step> {
    use crate::steps::Step;
    use alloc::string::ToString;
    let counter = ksn.transaction_counter();

    let ksn_bytes = ksn.as_bytes();
    let mut ksn_reg = [0u8; 8];
    ksn_reg.copy_from_slice(&ksn_bytes[2..10]);
    let tail = u32::from_be_bytes([ksn_reg[4], ksn_reg[5], ksn_reg[6], ksn_reg[7]]) & !COUNTER_MASK;
    ksn_reg[4..8].copy_from_slice(&tail.to_be_bytes());

    let mut out = alloc::vec::Vec::new();
    let mut cur_key = *ipek.as_bytes();
    out.push(Step::new(
        "IPEK",
        &cur_key,
        "initial key; ladder walks the set bits of the transaction counter",
    ));

    let mut shift_reg: u32 = 0x10_0000; // bit 20 (MSB of the 21-bit counter)
    let mut bit_index = 20u32;
    while shift_reg > 0 {
        if (shift_reg & counter) != 0 {
            let tail =
                u32::from_be_bytes([ksn_reg[4], ksn_reg[5], ksn_reg[6], ksn_reg[7]]) | shift_reg;
            ksn_reg[4..8].copy_from_slice(&tail.to_be_bytes());
            cur_key = generate_key(&cur_key, &ksn_reg);
            let mut label = alloc::string::String::from("generate @ bit ");
            label.push_str(&bit_index.to_string());
            out.push(Step::new(
                label,
                &cur_key,
                "counter bit set: derive the next key in the ladder (non-reversible)",
            ));
        }
        shift_reg >>= 1;
        bit_index = bit_index.saturating_sub(1);
    }

    out.push(Step::new(
        "transaction key",
        &cur_key,
        "final per-transaction key",
    ));
    out
}

/// Apply a DUKPT variant mask to a transaction key to obtain a working key.
pub fn apply_variant(key: &TransactionKey, variant: TdesVariant) -> TransactionKey {
    let mask = match variant {
        TdesVariant::Pin => &PIN_MASK,
        TdesVariant::Mac => &MAC_MASK,
        TdesVariant::Data => &DATA_MASK,
    };
    TransactionKey::new(xor16(key.as_bytes(), mask))
}
