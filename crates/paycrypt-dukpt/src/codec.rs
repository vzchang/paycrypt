//! Hex and BCD (binary-coded decimal) conversion helpers.

use alloc::string::String;
use alloc::vec::Vec;

/// Errors from hex/BCD conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CodecError {
    /// The input had an odd number of characters/nibbles.
    OddLength,
    /// The input contained a non-hexadecimal character.
    NonHex,
    /// The input contained a non-decimal-digit character.
    NonDigit,
}

impl core::fmt::Display for CodecError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CodecError::OddLength => f.write_str("odd-length input"),
            CodecError::NonHex => f.write_str("non-hex character"),
            CodecError::NonDigit => f.write_str("non-digit character"),
        }
    }
}

impl core::error::Error for CodecError {}

/// Decode an ASCII hex string (upper- or lower-case) into bytes.
pub fn from_hex(s: &str) -> Result<Vec<u8>, CodecError> {
    let bytes = s.as_bytes();
    if bytes.len() % 2 != 0 {
        return Err(CodecError::OddLength);
    }
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks_exact(2) {
        let hi = hex_val(pair[0])?;
        let lo = hex_val(pair[1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn hex_val(c: u8) -> Result<u8, CodecError> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(CodecError::NonHex),
    }
}

/// Encode bytes as an upper-case ASCII hex string.
pub fn to_hex_upper(bytes: &[u8]) -> String {
    const LUT: &[u8; 16] = b"0123456789ABCDEF";
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(LUT[(b >> 4) as usize] as char);
        s.push(LUT[(b & 0x0f) as usize] as char);
    }
    s
}

/// Pack an even-length decimal digit string into BCD bytes.
pub fn bcd_encode(digits: &str) -> Result<Vec<u8>, CodecError> {
    let bytes = digits.as_bytes();
    if bytes.len() % 2 != 0 {
        return Err(CodecError::OddLength);
    }
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks_exact(2) {
        let hi = digit_val(pair[0])?;
        let lo = digit_val(pair[1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn digit_val(c: u8) -> Result<u8, CodecError> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(CodecError::NonDigit),
    }
}

/// Decode BCD bytes into digits; a non-BCD nibble comes out as A-F, not an error.
pub fn bcd_decode(bytes: &[u8]) -> String {
    to_hex_upper(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrip() {
        let b = from_hex("0123ABcd").unwrap();
        assert_eq!(b, [0x01, 0x23, 0xAB, 0xCD]);
        assert_eq!(to_hex_upper(&b), "0123ABCD");
    }

    #[test]
    fn hex_rejects_odd_and_nonhex() {
        assert_eq!(from_hex("ABC"), Err(CodecError::OddLength));
        assert_eq!(from_hex("XY"), Err(CodecError::NonHex));
    }

    #[test]
    fn bcd_roundtrip() {
        assert_eq!(bcd_encode("1234").unwrap(), [0x12, 0x34]);
        assert_eq!(bcd_decode(&[0x12, 0x34]), "1234");
    }

    #[test]
    fn bcd_rejects_odd_and_nondigit() {
        assert_eq!(bcd_encode("123"), Err(CodecError::OddLength));
        assert_eq!(bcd_encode("12AB"), Err(CodecError::NonDigit));
    }

    proptest::proptest! {
        #[test]
        fn prop_hex_roundtrip(bytes in proptest::collection::vec(0u8..=255, 0..64)) {
            let s = to_hex_upper(&bytes);
            proptest::prop_assert_eq!(from_hex(&s).unwrap(), bytes);
        }
    }
}
