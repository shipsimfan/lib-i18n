use crate::{FluentBlockPlaceable, FluentInlinePlaceable, FluentPosition};

impl FluentBlockPlaceable {
    /// Gets the position of the start of this placeable
    pub const fn position(&self) -> FluentPosition {
        self.placeable.position()
    }

    /// Gets the contained placeable
    pub const fn placeable(&self) -> &FluentInlinePlaceable {
        &self.placeable
    }
}
