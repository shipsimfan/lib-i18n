use crate::fluent::{FluentPatternElement, Parse, Stream};

impl Parse for FluentPatternElement {
    fn parse(stream: &mut Stream) -> Option<Self> {
        if let Some(inline_text) = stream.step_parse() {
            return Some(FluentPatternElement::InlineText(inline_text));
        }

        if let Some(block_text) = stream.step_parse() {
            return Some(FluentPatternElement::BlockText(block_text));
        }

        None
    }
}
