use crate::fluent::{
    FluentBlockText, FluentInlinePlaceable, FluentInlineText, FluentPatternElement,
};

use super::FluentBlockPlaceable;

impl From<FluentInlineText> for FluentPatternElement {
    fn from(inline_text: FluentInlineText) -> Self {
        FluentPatternElement::InlineText(inline_text)
    }
}

impl From<FluentBlockText> for FluentPatternElement {
    fn from(block_text: FluentBlockText) -> Self {
        FluentPatternElement::BlockText(block_text)
    }
}

impl From<FluentInlinePlaceable> for FluentPatternElement {
    fn from(inline_placeable: FluentInlinePlaceable) -> Self {
        FluentPatternElement::InlinePlaceable(inline_placeable)
    }
}

impl From<FluentBlockPlaceable> for FluentPatternElement {
    fn from(block_placeable: FluentBlockPlaceable) -> Self {
        FluentPatternElement::BlockPlaceable(block_placeable)
    }
}
