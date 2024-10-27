use crate::fluent::{FluentInlineExpression, FluentInlinePlaceable};

impl From<FluentInlinePlaceable> for FluentInlineExpression {
    fn from(inline_placeable: FluentInlinePlaceable) -> Self {
        FluentInlineExpression::InlinePlaceable(Box::new(inline_placeable))
    }
}

impl From<Box<FluentInlinePlaceable>> for FluentInlineExpression {
    fn from(inline_placeable: Box<FluentInlinePlaceable>) -> Self {
        FluentInlineExpression::InlinePlaceable(inline_placeable)
    }
}
