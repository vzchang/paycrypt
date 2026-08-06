# paycrypt

Rust implementation of payments cryptography, built from public
standards and public test vectors:

- **`paycrypt-dukpt`**: ANSI X9.24 DUKPT (Derived Unique Key Per Transaction)
  for both TDES (X9.24-1) and AES (X9.24-3).
- **`paycrypt-pinblock`**: ISO 9564 PIN block formats 0-4.

Dual-licensed **MIT OR Apache-2.0**.

> **Educational and validation use only.** Not for production and not audited.
> Never use real keys, PANs, or PINs. See [SECURITY.md](SECURITY.md) for the
> full list of non-goals.

## Why this exists

Most maintained DUKPT implementations live in other languages (C, Go, Ruby,
C#, Java, Python). `paycrypt` is a permissively licensed, test-vector-validated
Rust implementation of both the legacy TDES and modern AES DUKPT schemes.

## Correctness

Correctness is treated as a first-class deliverable:

- **Known-answer tests** against published vectors. Every KAT asserts exact
  hex equality, so a passing test means byte-for-byte agreement with the
  standard.
  - TDES: BDK `0123…3210` + KSN `FFFF9876543210E00008` → IPEK
    `6AC292FAA1315B4D858AB3A3D7D5933A` (reconciled across moov-io/dukpt and
    sgbj/Dukpt.NET), plus per-transaction keys for counters 1-3.
  - AES: BDK-128 `FEDCBA…F1F1` + IK ID `1234567890123456` → Initial Key
    `1273671EA26AC29AFA4D1084127652A1`, plus PIN working keys for counter 1 and
    counter 8675309 (`0x845FED`, a multi-bit counter that exercises the full
    intermediate-key ladder).
  - ISO 9564: format-0/2/3 clear blocks and the AES format-4 example from the
    X9.24-3 supplement (encrypted block `A912150391AB65A67E52883D81CE2D15`),
    all against `psec`.
- **Property tests** (`proptest`): round-trips and derivation determinism.
- **Differential tests**: gated (`#[ignore]`) comparisons against independent
  implementations (moov-io/dukpt v1.0.0 for DUKPT, psec 1.3.0 for PIN blocks),
  run on every push by a dedicated CI job; see `scripts/`.
- **Fuzzing** (`cargo-fuzz`): every byte-parsing entry point.

Format 1 has no public known-answer vector, so it is round-trip-verified only
(never KAT-verified); this is called out in the code.

## Design

- Two sibling domain crates on top of audited [RustCrypto](https://github.com/RustCrypto)
  primitives (`aes`, `des`, `cipher`); the payments logic is
  hand-implemented, the ciphers are not.
- `no_std`-first; `#![forbid(unsafe_code)]` in both domain crates.
- Key material lives in newtypes that zeroize on drop and redact their `Debug`
  output; secret comparisons use `subtle`'s constant-time equality.
- PIN block codecs are split by contract: a pure clear-block codec for formats
  0-3, and a separate enciphering codec for format 4 (whose PAN block is
  interleaved between two AES passes).

## Live explainer

An interactive explainer (`site/`) runs the **real compiled library** in the
browser via WebAssembly and animates the DUKPT key ladder and ISO-0 PIN block
construction, exposing the intermediate derivation state that public
calculators hide. Every displayed value comes from a WASM call into the actual
crates, and the WASM step traces are pinned to the library's published test
vectors.

- WASM crate (`crates/paycrypt-wasm`) builds to a ~40 KB `.wasm` via
  `wasm-pack`; step-DTO logic is covered by native tests.
- Frontend is a Vite + Svelte app. Build and run it with Node 16+:
  `cd site && npm install && npm run dev` (see [`site/README.md`](site/README.md)).
- Live at **https://vzchang.github.io/paycrypt/**, deployed to GitHub Pages by
  `.github/workflows/site.yml` on every push to `main`.

## Status

Library v1: TDES + AES DUKPT and ISO 9564 PIN blocks 0-4, tested and CI-gated.
The explainer is live and redeploys on every push to `main`.

## Project docs

- [CHANGELOG.md](CHANGELOG.md): what has landed.
- [SECURITY.md](SECURITY.md): threat model and non-goals.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at
your option.
