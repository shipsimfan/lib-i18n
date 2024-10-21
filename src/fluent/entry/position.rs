use crate::fluent::{FluentEntry, FluentPosition};

impl FluentEntry {
    /// Gets the position of the start of this entry
    pub fn position(&self) -> FluentPosition {
        match self {
            FluentEntry::Junk(junk) => junk.position(),
        }
    }
}
