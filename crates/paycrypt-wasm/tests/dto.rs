//! Pins the WASM step traces to the library's published test vectors.

use paycrypt_wasm::{iso0_dtos, tdes_ladder_dtos};

#[test]
fn tdes_ladder_dtos_end_at_final_key() {
    // Library KAT: counter 3 transaction key
    let dtos =
        tdes_ladder_dtos("0123456789ABCDEFFEDCBA9876543210", "FFFF9876543210E00003").unwrap();
    assert!(dtos.len() >= 2, "IPEK + final at minimum");
    assert_eq!(dtos.first().unwrap().label, "IPEK");
    assert_eq!(dtos.last().unwrap().hex, "0DF3D9422ACA56E547676D07AD6BADFA");
    let last = dtos.last().unwrap();
    assert_eq!(
        last.hex,
        last.bytes
            .iter()
            .map(|b| format!("{b:02X}"))
            .collect::<String>()
    );
}

#[test]
fn iso0_dtos_end_at_clear_block() {
    // Library KAT
    let dtos = iso0_dtos("1234", "5555555551234567").unwrap();
    assert_eq!(dtos.len(), 3);
    assert_eq!(dtos.last().unwrap().hex, "041261AAAAEDCBA9");
}

#[test]
fn bad_input_is_err_not_panic() {
    assert!(tdes_ladder_dtos("zz", "FFFF9876543210E00003").is_err());
    assert!(tdes_ladder_dtos("0123456789ABCDEFFEDCBA9876543210", "FFFF").is_err());
    assert!(iso0_dtos("12", "5555555551234567").is_err());
}
