use crate::fluent::{FluentAttribute, FluentIdentifier, FluentPattern, FluentPosition};

impl FluentAttribute {
    /// Gets the position of the start of this attribute
    pub const fn position(&self) -> FluentPosition {
        self.name.position()
    }

    /// Gets the name of this attribute, not including leading '.'
    pub const fn name(&self) -> &FluentIdentifier {
        &self.name
    }

    /// Gets the pattern that defines this attribute
    pub const fn pattern(&self) -> &FluentPattern {
        &self.pattern
    }
}
