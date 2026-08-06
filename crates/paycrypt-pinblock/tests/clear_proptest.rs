//! Format then parse recovers the PIN for every clear PIN block format.

use paycrypt_pinblock::clear::{ClearPinBlockCodec, Iso0, Iso1, Iso2, Iso3};
use paycrypt_pinblock::types::{FixedRng, Pan, Pin};
use proptest::prelude::*;

/// A PIN of 4-12 decimal digits.
fn pin_strategy() -> impl Strategy<Value = String> {
    "[0-9]{4,12}"
}

/// A PAN of 13-19 decimal digits.
fn pan_strategy() -> impl Strategy<Value = String> {
    "[0-9]{13,19}"
}

proptest! {
    #[test]
    fn prop_iso0_roundtrip(pin_s in pin_strategy(), pan_s in pan_strategy()) {
        let pin = Pin::new(&pin_s).unwrap();
        let pan = Pan::new(&pan_s).unwrap();
        let mut rng = FixedRng::new(&[]);
        let block = Iso0.format(&pin, Some(&pan), &mut rng).unwrap();
        prop_assert_eq!(Iso0.parse(&block, Some(&pan)).unwrap(), pin);
    }

    #[test]
    fn prop_iso3_roundtrip(pin_s in pin_strategy(), pan_s in pan_strategy(), seed in any::<[u8; 8]>()) {
        let pin = Pin::new(&pin_s).unwrap();
        let pan = Pan::new(&pan_s).unwrap();
        let mut rng = FixedRng::new(&seed);
        let block = Iso3.format(&pin, Some(&pan), &mut rng).unwrap();
        prop_assert_eq!(Iso3.parse(&block, Some(&pan)).unwrap(), pin);
    }

    #[test]
    fn prop_iso2_roundtrip(pin_s in pin_strategy()) {
        let pin = Pin::new(&pin_s).unwrap();
        let mut rng = FixedRng::new(&[]);
        let block = Iso2.format(&pin, None, &mut rng).unwrap();
        prop_assert_eq!(Iso2.parse(&block, None).unwrap(), pin);
    }

    #[test]
    fn prop_iso1_roundtrip(pin_s in pin_strategy(), seed in any::<[u8; 8]>()) {
        let pin = Pin::new(&pin_s).unwrap();
        let mut rng = FixedRng::new(&seed);
        let block = Iso1.format(&pin, None, &mut rng).unwrap();
        prop_assert_eq!(Iso1.parse(&block, None).unwrap(), pin);
    }
}
