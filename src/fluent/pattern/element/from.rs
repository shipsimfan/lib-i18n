use crate::fluent::{FluentBlockText, FluentInlineText, FluentPatternElement};

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
