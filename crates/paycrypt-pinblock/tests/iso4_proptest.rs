//! Decipher after encipher recovers the PIN for any key and random tail.

use paycrypt_pinblock::iso4::{EncipheringPinBlockCodec, Iso4};
use paycrypt_pinblock::types::{FixedRng, Pan, Pin};
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_iso4_roundtrip(
        pin_s in "[0-9]{4,12}",
        // ISO 9564 format 4 PANs are 12-19 digits.
        pan_s in "[0-9]{12,19}",
        key in any::<[u8; 16]>(),
        tail in any::<[u8; 8]>(),
    ) {
        let pin = Pin::new(&pin_s).unwrap();
        let pan = Pan::new(&pan_s).unwrap();
        let mut rng = FixedRng::new(&tail);
        let block = Iso4.encipher(&key, &pin, &pan, &mut rng).unwrap();
        prop_assert_eq!(Iso4.decipher(&key, &block, &pan).unwrap(), pin);
    }
}
