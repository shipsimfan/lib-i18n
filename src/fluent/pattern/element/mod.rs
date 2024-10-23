mod inline_text;

mod display;
mod from;
mod get;
mod new;
mod parse;

pub use inline_text::FluentInlineText;

/// An element of a pattern describing a message
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluentPatternElement {
    /// The element is simple inline text
    InlineText(FluentInlineText),
}
