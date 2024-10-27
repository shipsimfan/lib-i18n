use crate::{FluentExpression, FluentInlinePlaceable};
use core::ops::Deref;

impl Deref for FluentInlinePlaceable {
    type Target = FluentExpression;

    fn deref(&self) -> &Self::Target {
        self.expression()
    }
}

impl AsRef<FluentExpression> for FluentInlinePlaceable {
    fn as_ref(&self) -> &FluentExpression {
        self.expression()
    }
}
