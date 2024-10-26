//! Utilities for parsing Fluent files

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
pub use expression::FluentExpression;
pub use identifier::FluentIdentifier;
pub use pattern::{
    FluentBlockText, FluentInlinePlaceable, FluentInlineText, FluentPattern, FluentPatternElement,
};
pub use resource::FluentResource;
