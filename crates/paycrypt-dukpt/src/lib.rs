#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! ANSI X9.24 DUKPT key derivation for TDES (X9.24-1) and AES (X9.24-3).
//!
//! **Educational and validation use only. Not for production, not audited.**

extern crate alloc;
