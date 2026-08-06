//! Validated PIN and PAN newtypes, the injected `Rng`, and the crate error type.

use alloc::string::String;
use core::fmt;
use subtle::ConstantTimeEq;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Minimum PIN length accepted (ISO 9564 permits 4-12 digit PINs).
pub const MIN_PIN_LEN: usize = 4;
/// Maximum PIN length accepted.
pub const MAX_PIN_LEN: usize = 12;

/// PIN block validation and codec errors; messages never include secret values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PinError {
    /// PIN length outside the ISO 9564 range of 4-12 digits.
    BadPinLength,
    /// A character was not an ASCII decimal digit.
    NonDigit,
    /// The chosen format needs a PAN but none was supplied.
    PanRequired,
    /// A block's control, length, or padding nibble did not match its format.
    BadBlock,
}

impl fmt::Display for PinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            PinError::BadPinLength => "PIN length must be 4-12 digits",
            PinError::NonDigit => "value must contain only decimal digits",
            PinError::PanRequired => "this PIN block format requires a PAN",
            PinError::BadBlock => "PIN block failed structural validation",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for PinError {}

/// A cardholder PIN of 4-12 digits; zeroized on drop, redacted in `Debug`, compared in constant time.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct Pin {
    digits: alloc::vec::Vec<u8>,
}

impl Pin {
    /// Validates and constructs a PIN from a decimal-digit string.
    pub fn new(pin: &str) -> Result<Self, PinError> {
        let bytes = pin.as_bytes();
        if bytes.len() < MIN_PIN_LEN || bytes.len() > MAX_PIN_LEN {
            return Err(PinError::BadPinLength);
        }
        if !bytes.iter().all(u8::is_ascii_digit) {
            return Err(PinError::NonDigit);
        }
        Ok(Self {
            digits: bytes.to_vec(),
        })
    }

    /// Number of PIN digits.
    pub fn len(&self) -> usize {
        self.digits.len()
    }

    /// Always `false`: a valid PIN has at least [`MIN_PIN_LEN`] digits.
    pub fn is_empty(&self) -> bool {
        self.digits.is_empty()
    }

    /// The PIN digits as ASCII bytes (`b'0'..=b'9'`).
    pub fn digits(&self) -> &[u8] {
        &self.digits
    }
}

impl fmt::Debug for Pin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Pin(<redacted>)")
    }
}

impl ConstantTimeEq for Pin {
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        // Length isn't secret; only the digits need a constant-time compare.
        if self.digits.len() != other.digits.len() {
            return subtle::Choice::from(0u8);
        }
        self.digits.ct_eq(&other.digits)
    }
}

impl PartialEq for Pin {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl Eq for Pin {}

/// A Primary Account Number: decimal digits only, no Luhn check.
#[derive(Clone, PartialEq, Eq)]
pub struct Pan {
    digits: String,
}

impl Pan {
    /// Validates and constructs a PAN from a decimal-digit string.
    pub fn new(pan: &str) -> Result<Self, PinError> {
        if pan.is_empty() || !pan.bytes().all(|b| b.is_ascii_digit()) {
            return Err(PinError::NonDigit);
        }
        Ok(Self {
            digits: String::from(pan),
        })
    }

    /// The PAN digits as an ASCII string slice.
    pub fn as_str(&self) -> &str {
        &self.digits
    }

    /// Number of PAN digits.
    pub fn len(&self) -> usize {
        self.digits.len()
    }

    /// Always `false`: a valid PAN has at least one digit.
    pub fn is_empty(&self) -> bool {
        self.digits.is_empty()
    }
}

impl fmt::Debug for Pan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // PANs are sensitive cardholder data; redact by default.
        f.write_str("Pan(<redacted>)")
    }
}

/// An 8-byte clear PIN block (ISO 9564 formats 0-3).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ClearPinBlock([u8; 8]);

impl ClearPinBlock {
    /// Wraps 8 raw bytes as a clear PIN block.
    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }

    /// The 8 block bytes.
    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }
}

impl fmt::Debug for ClearPinBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // A clear PIN block reveals the PIN when combined with the PAN; redact.
        f.write_str("ClearPinBlock(<redacted>)")
    }
}

/// Injected randomness, so the core stays `no_std`; real callers pass a CSPRNG.
pub trait Rng {
    /// Fills `dest` entirely with random bytes.
    fn fill(&mut self, dest: &mut [u8]);
}

/// Deterministic [`Rng`] that cycles a fixed byte sequence; tests only, not a CSPRNG.
pub struct FixedRng<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> FixedRng<'a> {
    /// Creates a `FixedRng` that repeats `bytes`; an empty slice yields zeros.
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
}

impl Rng for FixedRng<'_> {
    fn fill(&mut self, dest: &mut [u8]) {
        for b in dest.iter_mut() {
            if self.bytes.is_empty() {
                *b = 0;
            } else {
                *b = self.bytes[self.pos % self.bytes.len()];
                self.pos += 1;
            }
        }
    }
}
