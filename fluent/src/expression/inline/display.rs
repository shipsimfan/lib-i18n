use crate::FluentInlineExpression;

impl core::fmt::Display for FluentInlineExpression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FluentInlineExpression::InlinePlaceable(inline_placeable) => inline_placeable.fmt(f),
        }
    }
}
