use crate::fluent::{FluentAttribute, FluentMessage, FluentPattern, FluentPosition};

impl FluentMessage {
    /// Gets the position of the start of this message
    pub const fn position(&self) -> FluentPosition {
        self.name.position()
    }

    /// Gets the pattern defining this message
    pub const fn pattern(&self) -> Option<&FluentPattern> {
        self.pattern.as_ref()
    }

    /// Gets the attributes that are children of this message
    pub fn attributes(&self) -> &[FluentAttribute] {
        &self.attributes
    }
}
