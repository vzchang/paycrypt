#![no_main]
//! Fuzz the PIN block decode path: arbitrary 8-byte blocks + PAN must never panic.
use libfuzzer_sys::fuzz_target;
use paycrypt_pinblock::clear::{ClearPinBlockCodec, Iso0};
use paycrypt_pinblock::types::{ClearPinBlock, Pan};

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    let mut block = [0u8; 8];
    block.copy_from_slice(&data[..8]);
    let cpb = ClearPinBlock::from_bytes(block);
    if let Ok(pan) = core::str::from_utf8(&data[8..]) {
        if let Ok(pan) = Pan::new(pan) {
            let _ = Iso0.parse(&cpb, Some(&pan));
        }
    }
});
