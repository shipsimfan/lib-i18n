use crate::fluent::{FluentEntry, FluentPosition};

impl FluentEntry {
    /// Gets the position of the start of this entry
    pub fn position(&self) -> FluentPosition {
        match self {
            FluentEntry::Message(message) => message.position(),
            FluentEntry::Term(term) => term.position(),
            FluentEntry::Comment(comment) => comment.position(),
            FluentEntry::Junk(junk) => junk.position(),
        }
    }
}
