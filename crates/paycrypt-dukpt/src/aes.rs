//! AES DUKPT (ANSI X9.24-3) key derivation.

use crate::codec;
use aes::{Aes128, Aes192, Aes256};
use cipher::{BlockCipherEncrypt, KeyInit};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Key usage indicator (ANSI X9.24-3 §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyUsage {
    /// Key encryption key.
    Kek,
    /// PIN encryption working key.
    PinEncryption,
    /// MAC generation working key.
    MacGenerate,
    /// Data encryption working key.
    DataEncrypt,
    /// Intermediate/"future" key derivation.
    KeyDerivation,
    /// Initial-key derivation from the BDK.
    InitialKeyDerivation,
}

impl KeyUsage {
    /// The 2-byte usage code.
    pub fn code(self) -> u16 {
        match self {
            KeyUsage::Kek => 0x0002,
            KeyUsage::PinEncryption => 0x1000,
            KeyUsage::MacGenerate => 0x2000,
            KeyUsage::DataEncrypt => 0x3000,
            KeyUsage::KeyDerivation => 0x8000,
            KeyUsage::InitialKeyDerivation => 0x8001,
        }
    }
}

/// Key type / algorithm (ANSI X9.24-3 §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    /// Two-key Triple DES.
    Tdea2,
    /// Three-key Triple DES.
    Tdea3,
    /// AES-128.
    Aes128,
    /// AES-192.
    Aes192,
    /// AES-256.
    Aes256,
}

impl KeyType {
    /// The 2-byte algorithm code.
    pub fn algo_code(self) -> u16 {
        match self {
            KeyType::Tdea2 => 0x0000,
            KeyType::Tdea3 => 0x0001,
            KeyType::Aes128 => 0x0002,
            KeyType::Aes192 => 0x0003,
            KeyType::Aes256 => 0x0004,
        }
    }

    /// The key length in bits (packed into derivation data).
    pub fn key_bits(self) -> u16 {
        match self {
            KeyType::Tdea2 => 128,
            KeyType::Tdea3 => 192,
            KeyType::Aes128 => 128,
            KeyType::Aes192 => 192,
            KeyType::Aes256 => 256,
        }
    }

    /// The key length in bytes.
    pub fn byte_len(self) -> usize {
        match self {
            KeyType::Tdea2 => 16,
            KeyType::Tdea3 => 24,
            KeyType::Aes128 => 16,
            KeyType::Aes192 => 24,
            KeyType::Aes256 => 32,
        }
    }
}

/// Build the 16-byte AES-DUKPT derivation data block (ANSI X9.24-3 §6).
pub fn derivation_data(
    usage: KeyUsage,
    ktype: KeyType,
    block_counter: u8,
    ksn_data: &[u8; 8],
) -> [u8; 16] {
    let mut d = [0u8; 16];
    d[0] = 0x01;
    d[1] = block_counter;
    d[2..4].copy_from_slice(&usage.code().to_be_bytes());
    d[4..6].copy_from_slice(&ktype.algo_code().to_be_bytes());
    d[6..8].copy_from_slice(&ktype.key_bits().to_be_bytes());
    d[8..16].copy_from_slice(ksn_data);
    d
}

/// An AES DUKPT working or derived key (variable length).
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct AesWorkingKey(alloc::vec::Vec<u8>);

impl AesWorkingKey {
    /// Wrap raw key bytes.
    pub fn new(bytes: alloc::vec::Vec<u8>) -> Self {
        Self(bytes)
    }
    /// Borrow the raw key bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl core::fmt::Debug for AesWorkingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AesWorkingKey(<redacted>)")
    }
}

/// Error constructing an [`AesKsn`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AesKsnError {
    /// The KSN was not exactly 12 bytes / 24 hex characters.
    BadLength,
}

impl core::fmt::Display for AesKsnError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("AES KSN must be 12 bytes (24 hex chars)")
    }
}

impl core::error::Error for AesKsnError {}

/// An AES DUKPT Key Serial Number: 12 bytes = 8-byte Initial Key ID +
/// 4-byte (32-bit) transaction counter.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AesKsn([u8; 12]);

impl AesKsn {
    /// Parse from a 24-character hex string.
    pub fn from_hex(s: &str) -> Result<Self, AesKsnError> {
        let bytes = codec::from_hex(s).map_err(|_| AesKsnError::BadLength)?;
        let arr: [u8; 12] = bytes.as_slice().try_into().map_err(|_| AesKsnError::BadLength)?;
        Ok(Self(arr))
    }

    /// Construct directly from a 12-byte array.
    pub fn from_bytes(bytes: [u8; 12]) -> Self {
        Self(bytes)
    }

    /// The 8-byte Initial Key ID.
    pub fn initial_key_id(&self) -> [u8; 8] {
        let mut id = [0u8; 8];
        id.copy_from_slice(&self.0[..8]);
        id
    }

    /// The 32-bit transaction counter.
    pub fn transaction_counter(&self) -> u32 {
        u32::from_be_bytes([self.0[8], self.0[9], self.0[10], self.0[11]])
    }

    /// The raw 12 KSN bytes.
    pub fn as_bytes(&self) -> &[u8; 12] {
        &self.0
    }
}

impl core::fmt::Debug for AesKsn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AesKsn({})", codec::to_hex_upper(&self.0))
    }
}

fn aes_encrypt_block(key: &[u8], block: &[u8; 16]) -> [u8; 16] {
    let mut buf = *block;
    match key.len() {
        16 => Aes128::new_from_slice(key).unwrap().encrypt_block((&mut buf).into()),
        24 => Aes192::new_from_slice(key).unwrap().encrypt_block((&mut buf).into()),
        32 => Aes256::new_from_slice(key).unwrap().encrypt_block((&mut buf).into()),
        _ => panic!("invalid AES key length"),
    }
    buf
}

fn derive_key(key: &[u8], base_data: &[u8; 16], out_len: usize) -> alloc::vec::Vec<u8> {
    let mut out = alloc::vec::Vec::with_capacity(out_len);
    let mut block_counter: u8 = 1;
    while out.len() < out_len {
        let mut data = *base_data;
        data[1] = block_counter;
        let block = aes_encrypt_block(key, &data);
        out.extend_from_slice(&block);
        block_counter += 1;
    }
    out.truncate(out_len);
    out
}

/// Derive the Initial Key (IK) from the BDK and the 8-byte Initial Key ID.
pub fn derive_initial_key(bdk: &[u8], ik_id: &[u8; 8], ktype: KeyType) -> AesWorkingKey {
    let data = derivation_data(KeyUsage::InitialKeyDerivation, ktype, 1, ik_id);
    AesWorkingKey::new(derive_key(bdk, &data, ktype.byte_len()))
}

/// Derive a per-transaction working key from the Initial Key and KSN.
pub fn derive_working_key(
    ik: &[u8],
    ksn: &AesKsn,
    usage: KeyUsage,
    ktype: KeyType,
) -> AesWorkingKey {
    let ik_id = ksn.initial_key_id();
    let counter = ksn.transaction_counter();

    let mut cur = ik.to_vec();
    let mut applied: u32 = 0;
    let mut bit: u32 = 0x8000_0000;
    while bit > 0 {
        if (counter & bit) != 0 {
            applied |= bit;
            let mut ksn_data = [0u8; 8];
            ksn_data[..4].copy_from_slice(&ik_id[4..8]);
            ksn_data[4..8].copy_from_slice(&applied.to_be_bytes());
            // Intermediate derivations carry the IK's key type (X9.24-3).
            let data = derivation_data(KeyUsage::KeyDerivation, ktype, 1, &ksn_data);
            cur = derive_key(&cur, &data, ktype.byte_len());
        }
        bit >>= 1;
    }

    let mut ksn_data = [0u8; 8];
    ksn_data[..4].copy_from_slice(&ik_id[4..8]);
    ksn_data[4..8].copy_from_slice(&counter.to_be_bytes());
    let data = derivation_data(usage, ktype, 1, &ksn_data);
    AesWorkingKey::new(derive_key(&cur, &data, ktype.byte_len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Source: ASC X9.24-3 test-vector PDF and moov-io/dukpt aes_test.go.
    #[test]
    fn ik_derivation_data_matches_spec() {
        let ik_id = hex_literal::hex!("1234567890123456");
        let d = derivation_data(KeyUsage::InitialKeyDerivation, KeyType::Aes128, 1, &ik_id);
        assert_eq!(d, hex_literal::hex!("01018001000200801234567890123456"));
    }

    #[test]
    fn aes_ksn_splits() {
        let ksn = AesKsn::from_hex("123456789012345600000008").unwrap();
        assert_eq!(ksn.initial_key_id(), hex_literal::hex!("1234567890123456"));
        assert_eq!(ksn.transaction_counter(), 8);
    }
}
