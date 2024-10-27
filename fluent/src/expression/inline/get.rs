use crate::{FluentInlineExpression, FluentPosition};

impl FluentInlineExpression {
    /// Gets the position of the start of this inline expression
    pub const fn position(&self) -> FluentPosition {
        match self {
            FluentInlineExpression::InlinePlaceable(inline_placeable) => {
                inline_placeable.position()
            }
        }
    }
}
