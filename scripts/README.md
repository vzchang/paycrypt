# Differential-testing reference shims

The differential tests (`crates/*/tests/differential.rs`, marked `#[ignore]`)
compare paycrypt's output byte-for-byte against independent public
implementations over random inputs. They are `#[ignore]` by default so the
normal `cargo test` run does not require these external toolchains; a dedicated
CI job installs them and runs `cargo test -- --ignored`.

## Reference matrix

| Domain | Reference | Why |
|---|---|---|
| TDES + AES DUKPT | `moov-io/dukpt` (Go) | Single toolchain covers both DUKPT variants |
| ISO 9564 PIN blocks 0/2/3/4 | `psec` (Python) | Mature; covers formats 0/2/3/4 (not format 1) |

## Pinned versions

- Go module: `github.com/moov-io/dukpt@v1.0.0` (pinned in CI).
- Python: `psec==1.3.0` (pinned in CI).

## Shims

- `ref_moov.go`: reads `MODE ARG...` on argv, prints the reference hex on stdout.
- `ref_psec.py`: reads `MODE ARG...` on argv, prints the reference hex on stdout.

Both are intentionally tiny: parse argv, call the reference library, print
uppercase hex. The Rust differential tests shell out to them via
`std::process::Command` and compare.

The `differential` CI job builds both shims and runs these tests on every push;
treat it as the authoritative cross-implementation check.
