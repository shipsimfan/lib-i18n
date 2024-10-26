mod block_text;
mod inline_placeable;
mod inline_text;

mod display;
mod from;
mod get;
mod new;
mod parse;

pub use block_text::FluentBlockText;
pub use inline_placeable::FluentInlinePlaceable;
pub use inline_text::FluentInlineText;

/// An element of a pattern describing a message
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FluentPatternElement {
    /// The element is simple inline text
    InlineText(FluentInlineText),

    /// The element is a text element starting on another line
    BlockText(FluentBlockText),

    /// The element is a placeable that should be replaced based on some data
    InlinePlaceable(FluentInlinePlaceable),
}
