use crate::{FluentInlineText, FluentPosition};

impl FluentInlineText {
    /// Gets the position of the start of this inline text
    pub const fn position(&self) -> FluentPosition {
        self.position
    }

    /// Gets the content of the inline text
    pub fn content(&self) -> &str {
        &self.content
    }
}
