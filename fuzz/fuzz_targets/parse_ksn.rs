#![no_main]
//! Fuzz the KSN parsers (TDES + AES): arbitrary bytes must never panic.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    let _ = paycrypt_dukpt::types::TdesKsn::from_hex(&s);
    let _ = paycrypt_dukpt::aes::AesKsn::from_hex(&s);
    let _ = paycrypt_dukpt::types::TdesKsn::from_slice(data);
});
