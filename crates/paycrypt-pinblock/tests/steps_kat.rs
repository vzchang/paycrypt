//! The last step of instrumented ISO-0 construction equals the one-shot output.

use paycrypt_pinblock::clear::{format_iso0_steps, ClearPinBlockCodec, Iso0};
use paycrypt_pinblock::types::{FixedRng, Pan, Pin};

#[test]
fn iso0_steps_final_equals_oneshot() {
    for (pin_s, pan_s) in [
        ("1234", "5555555551234567"),
        ("123456789012", "5555555551234567"),
        ("1234", "5544332211009966"),
    ] {
        let pin = Pin::new(pin_s).unwrap();
        let pan = Pan::new(pan_s).unwrap();
        let (steps, block) = format_iso0_steps(&pin, &pan);
        let oneshot = Iso0
            .format(&pin, Some(&pan), &mut FixedRng::new(&[]))
            .unwrap();
        assert_eq!(block.as_bytes(), oneshot.as_bytes(), "block != one-shot");
        assert_eq!(
            &steps.last().unwrap().bytes,
            oneshot.as_bytes(),
            "final step != one-shot for PIN {pin_s} / PAN {pan_s}"
        );
        assert_eq!(steps.len(), 3, "expected PIN, account, XOR frames");
    }
}
