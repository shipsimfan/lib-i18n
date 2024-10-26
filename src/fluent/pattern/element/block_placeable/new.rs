use crate::fluent::{FluentBlockPlaceable, FluentInlinePlaceable};

impl FluentBlockPlaceable {
    /// Creates a new [`FluentBlockPlaceable`]
    pub fn new<T: Into<FluentInlinePlaceable>>(placeable: T) -> Self {
        FluentBlockPlaceable {
            placeable: placeable.into(),
        }
    }
}
