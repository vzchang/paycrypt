# Changelog

All notable changes to paycrypt are recorded here. Format loosely follows
[Keep a Changelog](https://keepachangelog.com/); the project is pre-1.0.

## [Unreleased]

### Added
- `paycrypt-dukpt`: ANSI X9.24-1 TDES DUKPT: IPEK derivation, the
  transaction-key ladder, PIN/MAC/Data variant keys, and KSN counter-advance
  with the ANSI popcount skip rule.
- `paycrypt-dukpt`: ANSI X9.24-3 AES DUKPT: initial-key derivation and the
  working-key ladder (AES-128/192/256, 2TDEA/3TDEA), validated against the
  X9.24-3 test-vector supplement.
- `paycrypt-pinblock`: ISO 9564 PIN block formats 0-4, with a pure clear-block
  codec (formats 0-3) and a separate enciphering codec for format 4.
- `paycrypt-wasm`: WebAssembly bindings exposing step-instrumented derivations
  (`tdes_ladder_steps`, `iso0_steps`) so a browser can run the real library.
- Interactive explainer site (`site/`): DUKPT ladder, PIN-block stepper, a
  two-terminals "same key from different secrets" demo, and an XOR playground,
  all computing values via the compiled WASM library.
- Test surface: known-answer tests against published vectors (byte-exact),
  property tests, a gated differential-testing harness (moov-io + psec), and
  cargo-fuzz targets for the parsing surfaces.

### Fixed
- AES DUKPT `derive_initial_key` / `derive_working_key` now return
  `Result<_, AesError>` instead of panicking on a key whose length is not
  16/24/32 bytes (found in code review).
- ISO 9564 format-4 `build_pan_block` now rejects PANs outside the 12-19 digit
  range with `PinError::BadBlock` instead of indexing past its buffer (found in
  code review).

### Notes
- Educational / validation use only; not for production, not PCI-compliant,
  not audited. See `SECURITY.md`.
