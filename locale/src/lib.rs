//! Utilities for getting and managing locales

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod language_tag;

#[cfg(feature = "current")]
mod current;

pub use language_tag::*;

#[cfg(feature = "current")]
pub use current::*;
