use crate::{FluentInlineExpression, Parse, Stream};

impl Parse for FluentInlineExpression {
    fn parse(stream: &mut Stream) -> Option<Self> {
        if let Some(variable_reference) = stream.step_parse() {
            return Some(FluentInlineExpression::VariableReference(
                variable_reference,
            ));
        }

        if let Some(inline_placeable) = stream.step_parse() {
            return Some(FluentInlineExpression::InlinePlaceable(inline_placeable));
        }

        None
    }
}
