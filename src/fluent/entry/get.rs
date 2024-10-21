use crate::fluent::{FluentEntry, FluentPosition};

impl FluentEntry {
    /// Gets the position of the start of this entry
    pub fn position(&self) -> FluentPosition {
        match self {
            FluentEntry::Comment(comment) => comment.position(),
            FluentEntry::Junk(junk) => junk.position(),
        }
    }
}
