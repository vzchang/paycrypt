//! ISO 9564-1:2017 §9.4.2 format 4 (AES) PIN blocks.
//!
//! Exposed as `encipher`/`decipher` rather than `format`/`parse` because the
//! PAN block is XORed in between two AES passes.

use aes::Aes128;
use cipher::array::Array;
use cipher::{BlockCipherDecrypt, BlockCipherEncrypt, KeyInit};

use crate::types::{Pan, Pin, PinError, Rng};

/// A 16-byte enciphered ISO 9564 format-4 PIN block.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EncryptedPinBlock([u8; 16]);

impl EncryptedPinBlock {
    /// Wraps 16 raw bytes as an enciphered PIN block.
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// The 16 enciphered block bytes.
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl core::fmt::Debug for EncryptedPinBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("EncryptedPinBlock(<redacted>)")
    }
}

/// A codec for the 16-byte AES-enciphered ISO 9564 format-4 PIN block.
pub trait EncipheringPinBlockCodec {
    /// Enciphers `pin` bound to `pan` under a 16-byte AES-128 `key`.
    fn encipher(
        &self,
        key: &[u8; 16],
        pin: &Pin,
        pan: &Pan,
        rng: &mut dyn Rng,
    ) -> Result<EncryptedPinBlock, PinError>;

    /// Deciphers a format-4 block under `key`, bound to `pan`, recovering the PIN.
    fn decipher(
        &self,
        key: &[u8; 16],
        block: &EncryptedPinBlock,
        pan: &Pan,
    ) -> Result<Pin, PinError>;
}

/// ISO 9564 format 4 (AES) enciphering codec.
pub struct Iso4;

fn build_pin_field(pin: &Pin, rng: &mut dyn Rng) -> Result<[u8; 16], PinError> {
    let digits = pin.digits();
    if digits.len() < 4 || digits.len() > 12 {
        return Err(PinError::BadPinLength);
    }
    let mut nibbles = [0u8; 32];
    nibbles[0] = 0x04;
    nibbles[1] = digits.len() as u8;
    for (i, d) in digits.iter().enumerate() {
        nibbles[2 + i] = d - b'0';
    }
    for n in nibbles.iter_mut().take(16).skip(2 + digits.len()) {
        *n = 0x0A;
    }
    let mut field = [0u8; 16];
    for i in 0..8 {
        field[i] = (nibbles[2 * i] << 4) | (nibbles[2 * i + 1] & 0x0F);
    }
    rng.fill(&mut field[8..16]);
    Ok(field)
}

fn build_pan_block(pan: &Pan) -> Result<[u8; 16], PinError> {
    let pan_bytes = pan.as_str().as_bytes();
    // Format 4 PANs are 12-19 digits; anything else overflows M or the buffer.
    if pan_bytes.len() < 12 || pan_bytes.len() > 19 {
        return Err(PinError::BadBlock);
    }
    let m = (pan_bytes.len() - 12) as u8;
    let mut nibbles = [0u8; 32];
    nibbles[0] = m & 0x0F;
    for (i, d) in pan_bytes.iter().enumerate() {
        nibbles[1 + i] = d - b'0';
    }
    let mut block = [0u8; 16];
    for i in 0..16 {
        block[i] = (nibbles[2 * i] << 4) | (nibbles[2 * i + 1] & 0x0F);
    }
    Ok(block)
}

fn xor16(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = a[i] ^ b[i];
    }
    out
}

fn parse_pin_field(field: &[u8; 16]) -> Result<Pin, PinError> {
    if field[0] >> 4 != 0x04 {
        return Err(PinError::BadBlock);
    }
    let len = (field[0] & 0x0F) as usize;
    if !(4..=12).contains(&len) {
        return Err(PinError::BadBlock);
    }
    let mut digits = alloc::vec::Vec::with_capacity(len);
    for i in 0..len {
        let nibble_idx = 2 + i;
        let byte = field[nibble_idx / 2];
        let nibble = if nibble_idx % 2 == 0 {
            byte >> 4
        } else {
            byte & 0x0F
        };
        if nibble > 9 {
            return Err(PinError::BadBlock);
        }
        digits.push(b'0' + nibble);
    }
    let s = core::str::from_utf8(&digits).map_err(|_| PinError::BadBlock)?;
    Pin::new(s)
}

impl EncipheringPinBlockCodec for Iso4 {
    fn encipher(
        &self,
        key: &[u8; 16],
        pin: &Pin,
        pan: &Pan,
        rng: &mut dyn Rng,
    ) -> Result<EncryptedPinBlock, PinError> {
        let cipher = Aes128::new(&Array(*key));
        let pin_field = build_pin_field(pin, rng)?;
        let pan_block = build_pan_block(pan)?;

        let mut a = Array(pin_field);
        cipher.encrypt_block(&mut a);

        let b_arr = xor16(&a.0, &pan_block);

        let mut b = Array(b_arr);
        cipher.encrypt_block(&mut b);
        Ok(EncryptedPinBlock(b.0))
    }

    fn decipher(
        &self,
        key: &[u8; 16],
        block: &EncryptedPinBlock,
        pan: &Pan,
    ) -> Result<Pin, PinError> {
        let cipher = Aes128::new(&Array(*key));
        let pan_block = build_pan_block(pan)?;

        let mut b = Array(*block.as_bytes());
        cipher.decrypt_block(&mut b);

        let a_arr = xor16(&b.0, &pan_block);

        let mut a = Array(a_arr);
        cipher.decrypt_block(&mut a);

        parse_pin_field(&a.0)
    }
}
