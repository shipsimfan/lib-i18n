use crate::fluent::{FluentInlineText, FluentPatternElement};

impl From<FluentInlineText> for FluentPatternElement {
    fn from(inline_text: FluentInlineText) -> Self {
        FluentPatternElement::InlineText(inline_text)
    }
}
