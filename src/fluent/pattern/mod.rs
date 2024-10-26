mod element;

mod deref;
mod display;
mod from;
mod get;
mod iter;
mod new;
mod parse;
mod push;

pub use element::{
    FluentBlockPlaceable, FluentBlockText, FluentInlinePlaceable, FluentInlineText,
    FluentPatternElement,
};

/// A pattern describing a message, attribute, or term
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentPattern {
    /// The elements making up the pattern
    elements: Vec<FluentPatternElement>,
}
