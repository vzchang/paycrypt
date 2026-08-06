//! Differential tests against the psec Python reference; run with `cargo test -- --ignored`.

use paycrypt_pinblock::clear::{ClearPinBlockCodec, Iso0};
use paycrypt_pinblock::types::{FixedRng, Pan, Pin};
use std::process::Command;

fn to_hex_upper(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

fn psec_iso0(pin: &str, pan: &str) -> String {
    // Tests run with the crate dir as CWD; the shim lives at the repo root.
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../scripts/ref_psec.py");
    let out = Command::new("python3")
        .args([script, "iso0", pin, pan])
        .output()
        .expect("run ref_psec.py");
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

#[test]
#[ignore = "requires python3 + psec; run via the differential CI job"]
fn iso0_matches_psec() {
    let cases = [
        ("1234", "5555555551234567"),
        ("987654", "4111111111111111"),
        ("12345678", "4012345678909"),
    ];
    for (pin_s, pan_s) in cases {
        let pin = Pin::new(pin_s).unwrap();
        let pan = Pan::new(pan_s).unwrap();
        let ours = Iso0
            .format(&pin, Some(&pan), &mut FixedRng::new(&[]))
            .unwrap();
        let ours_hex = to_hex_upper(ours.as_bytes());
        assert_eq!(ours_hex, psec_iso0(pin_s, pan_s), "ISO-0 mismatch vs psec");
    }
}
