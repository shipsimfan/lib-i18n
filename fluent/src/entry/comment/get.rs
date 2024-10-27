use crate::{FluentComment, FluentPosition};

impl FluentComment {
    /// Gets the position of the start of this comment
    pub const fn position(&self) -> FluentPosition {
        self.position
    }

    /// Gets the number of `#` characters prefixing the comment
    pub const fn hashes(&self) -> u8 {
        self.hashes
    }

    /// Gets the content of the comment
    pub fn content(&self) -> &str {
        &self.content
    }
}
