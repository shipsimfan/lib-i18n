use crate::{FluentIdentifier, FluentPosition, FluentVariableReference};

impl FluentVariableReference {
    /// Gets the position of the start of this reference
    pub const fn position(&self) -> FluentPosition {
        self.position
    }

    /// Gets the name of the variable being referenced
    pub const fn name(&self) -> &FluentIdentifier {
        &self.name
    }
}
