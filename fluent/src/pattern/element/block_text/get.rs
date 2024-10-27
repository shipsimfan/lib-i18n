use crate::{FluentBlockText, FluentPosition};

impl FluentBlockText {
    /// Gets the position of the start of the text
    pub const fn position(&self) -> FluentPosition {
        self.position
    }

    /// Gets the content of the text
    pub fn content(&self) -> &str {
        &self.content
    }
}
