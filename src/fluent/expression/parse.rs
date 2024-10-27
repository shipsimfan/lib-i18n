use crate::fluent::{FluentExpression, Parse, Stream};

impl Parse for FluentExpression {
    fn parse(stream: &mut Stream) -> Option<Self> {
        if let Some(inline_expression) = stream.step_parse() {
            return Some(FluentExpression::Inline(inline_expression));
        }

        None
    }
}
