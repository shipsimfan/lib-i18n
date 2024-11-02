use crate::{FluentAttribute, FluentIdentifier, FluentMessage, FluentPattern, FluentPosition};

impl FluentMessage {
    /// Gets the position of the start of this message
    pub const fn position(&self) -> FluentPosition {
        self.name.position()
    }

    /// Gets the name of the message
    pub const fn name(&self) -> &FluentIdentifier {
        &self.name
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
