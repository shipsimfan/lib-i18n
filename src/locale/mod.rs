//! Utilities for getting and managing locales

mod language_tag;

#[cfg(feature = "current")]
mod current;

pub use language_tag::*;

#[cfg(feature = "current")]
pub use current::*;
