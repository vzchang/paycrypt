# Security policy

## Scope and intent

`paycrypt` is an **educational / validation** implementation of payments
cryptography, built from public standards. It exists to teach and to
validate other implementations against published test vectors. It is **not** a
production payment-security product.

## Explicit non-goals

- **No HSM / TRSM custody.** Real DUKPT relies on tamper-resistant hardware for
  key injection, storage, and tamper response. This library provides none of
  that.
- **No constant-time guarantee for the TDES path.** The underlying RustCrypto
  `des` crate is explicitly not constant-time. The pure-software TDES/PIN path
  must never guard real cardholder data.
- **Not PCI-compliant, not FIPS-validated, not CAVP-validated, not audited.**
  Passing published test vectors demonstrates functional correctness against
  the standard; it does not constitute formal validation.

## Data handling

- **Never use real BDKs, PANs, or PINs, even briefly.** Use only synthetic
  test PANs and the key material published in the standards' own test vectors.
- Key and PIN types are constructed only from caller-supplied bytes, wrapped in
  newtypes that zeroize on drop and redact their `Debug` output. Secret
  comparisons use constant-time equality (`subtle`). Note that `zeroize` cannot
  erase copies left by reallocation, and constant-time guarantees hold only in
  release builds.

## Reporting

This is a personal educational project. If you find a correctness bug (e.g. a
vector mismatch), please open an issue with the input and the expected vs.
produced output.
