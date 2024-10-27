use crate::{FluentIdentifier, FluentPosition};

impl FluentIdentifier {
    /// Gets the position of the start of this identifier
    pub const fn position(&self) -> FluentPosition {
        self.position
    }

    /// Gets the content of this identifier
    pub fn content(&self) -> &str {
        &self.content
    }
}
