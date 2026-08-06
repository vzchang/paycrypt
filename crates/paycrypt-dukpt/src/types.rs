//! Zeroizing key and Key Serial Number (KSN) newtypes for TDES DUKPT.

use crate::codec;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Clears the 21-bit counter from a KSN, leaving the IKSN (ANSI X9.24-1).
const KSN_COUNTER_CLEAR: [u8; 10] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xE0, 0x00, 0x00,
];

const COUNTER_MASK: u32 = 0x1F_FFFF;

macro_rules! key_newtype {
    ($(#[$m:meta])* $name:ident) => {
        $(#[$m])*
        #[derive(Clone, Zeroize, ZeroizeOnDrop)]
        pub struct $name([u8; 16]);

        impl $name {
            /// Construct from raw 16-byte (double-length) key material.
            pub fn new(bytes: [u8; 16]) -> Self {
                Self(bytes)
            }

            /// Borrow the raw key bytes.
            pub fn as_bytes(&self) -> &[u8; 16] {
                &self.0
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(<redacted>)", stringify!($name))
            }
        }
    };
}

key_newtype!(
    /// Base Derivation Key: the acquirer-held master key.
    Bdk
);
key_newtype!(
    /// Initial PIN Encryption Key (IPEK), injected into a device.
    InitialKey
);
key_newtype!(
    /// A per-transaction key produced by walking the DUKPT key ladder.
    TransactionKey
);

/// Errors constructing a [`TdesKsn`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum KsnError {
    /// The KSN was not exactly 10 bytes / 20 hex characters.
    BadLength,
}

impl core::fmt::Display for KsnError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("KSN must be 10 bytes (20 hex chars)")
    }
}

impl core::error::Error for KsnError {}

/// TDES DUKPT Key Serial Number (10 bytes): the IKSN plus a 21-bit transaction counter.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TdesKsn([u8; 10]);

impl TdesKsn {
    /// Parse a KSN from a 20-character hex string.
    pub fn from_hex(s: &str) -> Result<Self, KsnError> {
        let bytes = codec::from_hex(s).map_err(|_| KsnError::BadLength)?;
        Self::from_slice(&bytes)
    }

    /// Construct from a 10-byte slice.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, KsnError> {
        let arr: [u8; 10] = bytes.try_into().map_err(|_| KsnError::BadLength)?;
        Ok(Self(arr))
    }

    /// Construct directly from a 10-byte array.
    pub fn from_bytes(bytes: [u8; 10]) -> Self {
        Self(bytes)
    }

    /// The raw 10 KSN bytes.
    pub fn as_bytes(&self) -> &[u8; 10] {
        &self.0
    }

    /// The 21-bit transaction counter (the low 21 bits of the KSN).
    pub fn transaction_counter(&self) -> u32 {
        let tail = u32::from_be_bytes([self.0[6], self.0[7], self.0[8], self.0[9]]);
        tail & COUNTER_MASK
    }

    /// The leading 8 bytes of the counter-cleared KSN, used to derive the IPEK.
    pub fn initial_key_id(&self) -> [u8; 8] {
        let mut masked = self.0;
        for i in 0..10 {
            masked[i] &= KSN_COUNTER_CLEAR[i];
        }
        let mut iksn = [0u8; 8];
        iksn.copy_from_slice(&masked[..8]);
        iksn
    }
}

impl TdesKsn {
    fn with_counter(&self, counter: u32) -> Self {
        let mut bytes = self.0;
        let cleared =
            u32::from_be_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]) & !COUNTER_MASK;
        let combined = cleared | (counter & COUNTER_MASK);
        bytes[6..10].copy_from_slice(&combined.to_be_bytes());
        Self(bytes)
    }

    /// Advance to the next counter with at most 10 set bits (ANSI X9.24-1), or `None` when exhausted.
    pub fn next_valid(&self) -> Option<Self> {
        let mut c = self.transaction_counter();
        loop {
            c += 1;
            if c > COUNTER_MASK {
                return None;
            }
            if is_valid_counter(c) {
                return Some(self.with_counter(c));
            }
        }
    }
}

/// Whether a counter fits in 21 bits with at most 10 set bits (ANSI X9.24-1).
pub fn is_valid_counter(counter: u32) -> bool {
    counter <= COUNTER_MASK && counter.count_ones() <= 10
}

impl core::fmt::Debug for TdesKsn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TdesKsn({})", codec::to_hex_upper(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Source: SoftwareVerde/java-dukpt and moov-io/dukpt.
    #[test]
    fn ksn_splits_counter_and_iksn() {
        let ksn = TdesKsn::from_hex("FFFF9876543210E00008").unwrap();
        assert_eq!(ksn.transaction_counter(), 8);
        assert_eq!(ksn.initial_key_id(), hex_literal::hex!("FFFF9876543210E0"));
    }

    #[test]
    fn ksn_rejects_bad_length() {
        assert_eq!(TdesKsn::from_hex("FFFF"), Err(KsnError::BadLength));
    }

    #[test]
    fn bdk_debug_is_redacted() {
        extern crate std;
        let b = Bdk::new([0u8; 16]);
        assert_eq!(std::format!("{b:?}"), "Bdk(<redacted>)");
    }

    #[test]
    fn counter_validity_and_skip() {
        assert!(!is_valid_counter(0x1F_FFFF));
        assert!(is_valid_counter(0x1));
        assert!(!is_valid_counter(0x20_0000));
        let ksn = TdesKsn::from_hex("FFFF9876543210E00007").unwrap();
        let next = ksn.next_valid().unwrap();
        assert!(next.transaction_counter() > 7);
        assert!(is_valid_counter(next.transaction_counter()));
        assert_eq!(next.initial_key_id(), ksn.initial_key_id());
    }
}
