use crate::fluent::FluentInlineExpression;

impl FluentInlineExpression {
    /// Creates a new [`FluentInlineExpression`]
    pub fn new<T: Into<FluentInlineExpression>>(expression: T) -> Self {
        expression.into()
    }
}
