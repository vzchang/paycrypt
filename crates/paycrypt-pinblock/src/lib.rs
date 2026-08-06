#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! ISO 9564 PIN block formats 0-4.
//!
//! **Educational and validation use only. Not for production, not audited.**

extern crate alloc;

pub mod clear;
pub mod iso4;
pub mod types;
