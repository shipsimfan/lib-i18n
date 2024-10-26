use crate::fluent::{FluentPatternElement, FluentPosition};

impl FluentPatternElement {
    /// Gets the position of the start of this pattern
    pub const fn position(&self) -> FluentPosition {
        match self {
            FluentPatternElement::InlineText(inline_text) => inline_text.position(),
            FluentPatternElement::BlockText(block_text) => block_text.position(),
        }
    }
}
