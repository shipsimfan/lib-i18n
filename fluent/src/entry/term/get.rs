use crate::{FluentAttribute, FluentPattern, FluentPosition, FluentTerm};

impl FluentTerm {
    /// Gets the position of the start of this term
    pub const fn position(&self) -> FluentPosition {
        self.position
    }

    /// Gets the pattern defining this term
    pub const fn pattern(&self) -> &FluentPattern {
        &self.pattern
    }

    /// Gets the attributes that are children of this term
    pub fn attributes(&self) -> &[FluentAttribute] {
        &self.attributes
    }
}
