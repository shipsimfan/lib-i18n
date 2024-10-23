use crate::fluent::FluentPatternElement;

impl core::fmt::Display for FluentPatternElement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FluentPatternElement::InlineText(inline_text) => inline_text.fmt(f),
        }
    }
}
