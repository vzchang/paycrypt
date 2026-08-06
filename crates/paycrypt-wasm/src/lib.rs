//! WASM bindings that run the real `paycrypt` library in the browser.
#![deny(missing_docs)]

use serde::Serialize;
use wasm_bindgen::prelude::*;

/// A serializable derivation step for the explainer UI.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct StepDto {
    /// Short label for the frame.
    pub label: String,
    /// The intermediate byte value at this step (serialized as a byte array).
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,
    /// Uppercase hex rendering of `bytes`, for convenient display.
    pub hex: String,
    /// Human-readable explanation.
    pub note: String,
}

fn to_hex_upper(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

/// The TDES DUKPT ladder as step DTOs, testable without a browser.
pub fn tdes_ladder_dtos(bdk_hex: &str, ksn_hex: &str) -> Result<Vec<StepDto>, String> {
    use paycrypt_dukpt::tdes::{derive_ipek, derive_transaction_key_steps};
    use paycrypt_dukpt::types::{Bdk, TdesKsn};

    let bdk_bytes = paycrypt_dukpt::codec::from_hex(bdk_hex).map_err(|_| "bad BDK hex")?;
    let bdk_arr: [u8; 16] = bdk_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "BDK must be 16 bytes")?;
    let bdk = Bdk::new(bdk_arr);
    let ksn = TdesKsn::from_hex(ksn_hex).map_err(|_| "bad KSN")?;

    let ipek = derive_ipek(&bdk, &ksn);
    let steps = derive_transaction_key_steps(&ipek, &ksn);
    Ok(steps
        .into_iter()
        .map(|s| StepDto {
            hex: to_hex_upper(&s.bytes),
            bytes: s.bytes,
            label: s.label,
            note: s.note,
        })
        .collect())
}

/// The ISO-0 PIN block assembly frames as step DTOs, testable without a browser.
pub fn iso0_dtos(pin: &str, pan: &str) -> Result<Vec<StepDto>, String> {
    use paycrypt_pinblock::clear::format_iso0_steps;
    use paycrypt_pinblock::types::{Pan, Pin};

    let pin = Pin::new(pin).map_err(|_| "bad PIN")?;
    let pan = Pan::new(pan).map_err(|_| "bad PAN")?;
    let (steps, _block) = format_iso0_steps(&pin, &pan);
    Ok(steps
        .into_iter()
        .map(|s| StepDto {
            hex: to_hex_upper(&s.bytes),
            bytes: s.bytes.to_vec(),
            label: s.label,
            note: s.note,
        })
        .collect())
}

/// Version string, to confirm the module loaded.
#[wasm_bindgen]
pub fn version() -> String {
    "paycrypt-wasm 0.0.0".to_string()
}

/// WASM entry point: TDES DUKPT ladder steps as a JS array of `StepDto`.
#[wasm_bindgen]
pub fn tdes_ladder_steps(bdk_hex: &str, ksn_hex: &str) -> Result<JsValue, JsValue> {
    let dtos = tdes_ladder_dtos(bdk_hex, ksn_hex).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// WASM entry point: ISO-0 PIN block assembly steps as a JS array of `StepDto`.
#[wasm_bindgen]
pub fn iso0_steps(pin: &str, pan: &str) -> Result<JsValue, JsValue> {
    let dtos = iso0_dtos(pin, pan).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}
