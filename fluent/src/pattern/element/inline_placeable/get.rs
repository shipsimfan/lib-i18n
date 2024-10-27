use crate::{FluentExpression, FluentInlinePlaceable, FluentPosition};

impl FluentInlinePlaceable {
    /// Gets the position of the start of this placeable
    pub const fn position(&self) -> FluentPosition {
        self.position
    }

    /// Gets the expression making up this placeable
    pub const fn expression(&self) -> &FluentExpression {
        &self.expression
    }
}
