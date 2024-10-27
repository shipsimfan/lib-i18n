use crate::{FluentInlineExpression, FluentInlinePlaceable, FluentVariableReference};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

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

impl From<FluentVariableReference> for FluentInlineExpression {
    fn from(variable_reference: FluentVariableReference) -> Self {
        FluentInlineExpression::VariableReference(variable_reference)
    }
}
