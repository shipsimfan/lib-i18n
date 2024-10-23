use crate::fluent::FluentPosition;

mod deref;
mod display;
mod get;
mod new;
mod parse;

/// A simple inline text pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentInlineText {
    /// The start of this inline text
    position: FluentPosition,

    /// The content of the pattern
    content: String,
}
