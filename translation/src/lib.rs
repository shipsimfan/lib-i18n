//! Utilities for displaying messages in different languages

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod argument;
mod display;
mod message;
mod message_key;

pub use argument::Argument;
pub use display::MessageDisplay;
pub use message::Message;
pub use message_key::MessageKey;
