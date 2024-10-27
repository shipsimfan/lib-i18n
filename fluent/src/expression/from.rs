use crate::{FluentExpression, FluentInlineExpression};

impl<T: Into<FluentInlineExpression>> From<T> for FluentExpression {
    fn from(inline_expression: T) -> Self {
        FluentExpression::Inline(inline_expression.into())
    }
}
