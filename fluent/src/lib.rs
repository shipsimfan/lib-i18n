//! Utilities for parsing Fluent files

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

extern crate alloc;

mod parse;
mod position;
mod stream;

mod attribute;
mod blank;
mod entry;
mod expression;
mod identifier;
mod line_end;
mod pattern;
mod resource;

use parse::Parse;
use stream::{Stream, StreamCollector};

pub use parse::parse;
pub use position::FluentPosition;

#[cfg(feature = "std")]
pub use parse::parse_file;

use blank::{Blank, BlankBlock, BlankInline};
use line_end::LineEnd;

pub use attribute::FluentAttribute;
pub use entry::*;
pub use expression::{FluentExpression, FluentInlineExpression};
pub use identifier::FluentIdentifier;
pub use pattern::{
    FluentBlockPlaceable, FluentBlockText, FluentInlinePlaceable, FluentInlineText, FluentPattern,
    FluentPatternElement,
};
pub use resource::FluentResource;
