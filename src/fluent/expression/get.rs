use crate::fluent::{FluentExpression, FluentPosition};

impl FluentExpression {
    /// Gets the position of the start of this expression
    pub const fn position(&self) -> FluentPosition {
        match self {
            FluentExpression::Inline(inline) => inline.position(),
        }
    }
}
