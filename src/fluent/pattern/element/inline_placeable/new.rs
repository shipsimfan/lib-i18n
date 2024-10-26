use crate::fluent::{FluentExpression, FluentInlinePlaceable, FluentPosition};

impl FluentInlinePlaceable {
    /// Creates a new [`FluentInlinePlaceable`]
    pub fn new<P: Into<FluentPosition>, T: Into<FluentExpression>>(
        position: P,
        expression: T,
    ) -> Self {
        let position = position.into();
        let expression = expression.into();

        assert!(position < expression.position());

        FluentInlinePlaceable {
            position,
            expression,
        }
    }
}
