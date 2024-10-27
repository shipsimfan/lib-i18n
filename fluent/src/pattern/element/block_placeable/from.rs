use crate::{FluentBlockPlaceable, FluentInlinePlaceable};

impl<T: Into<FluentInlinePlaceable>> From<T> for FluentBlockPlaceable {
    fn from(placeable: T) -> Self {
        FluentBlockPlaceable {
            placeable: placeable.into(),
        }
    }
}
