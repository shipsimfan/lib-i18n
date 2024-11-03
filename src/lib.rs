//! Framework for internationalization

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use i18n_locale as locale;
pub use i18n_translation as translation;

#[cfg(feature = "fluent")]
pub use i18n_fluent as fluent;

#[cfg(feature = "fluent-macro")]
pub use i18n_fluent_macro::include_fluent;
