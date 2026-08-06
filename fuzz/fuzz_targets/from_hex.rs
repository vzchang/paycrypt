#![no_main]
//! Fuzz the hex decoder: arbitrary bytes must never panic.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    let _ = paycrypt_dukpt::codec::from_hex(&s);
});
