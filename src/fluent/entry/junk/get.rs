use crate::fluent::{FluentJunk, FluentPosition};

impl FluentJunk {
    /// Gets the position of the start of this junk
    pub fn position(&self) -> FluentPosition {
        self.position
    }

    /// Gets the content of the junk
    pub fn content(&self) -> &str {
        &self.content
    }
}
