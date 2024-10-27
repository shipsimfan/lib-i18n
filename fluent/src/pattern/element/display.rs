use crate::FluentPatternElement;

impl core::fmt::Display for FluentPatternElement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FluentPatternElement::InlineText(inline_text) => inline_text.fmt(f),
            FluentPatternElement::BlockText(block_text) => block_text.fmt(f),
            FluentPatternElement::InlinePlaceable(inline_placeable) => inline_placeable.fmt(f),
            FluentPatternElement::BlockPlaceable(block_placeable) => block_placeable.fmt(f),
        }
    }
}
