use crate::FluentExpression;

impl FluentExpression {
    /// Creates a new [`FluentExpression`]
    pub fn new<T: Into<FluentExpression>>(expression: T) -> Self {
        expression.into()
    }
}
