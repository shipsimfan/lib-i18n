mod element;

mod deref;
mod display;
mod from;
mod get;
mod iter;
mod new;
mod parse;
mod push;

pub use element::{FluentBlockText, FluentInlineText, FluentPatternElement};

/// A pattern describing a message, attribute, or term
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluentPattern {
    /// The elements making up the pattern
    elements: Vec<FluentPatternElement>,
}
