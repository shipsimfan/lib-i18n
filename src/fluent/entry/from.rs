use crate::fluent::{FluentComment, FluentEntry, FluentJunk, FluentMessage, FluentTerm};

impl From<FluentMessage> for FluentEntry {
    fn from(message: FluentMessage) -> Self {
        FluentEntry::Message(message)
    }
}

impl From<FluentTerm> for FluentEntry {
    fn from(term: FluentTerm) -> Self {
        FluentEntry::Term(term)
    }
}

impl From<FluentComment> for FluentEntry {
    fn from(comment: FluentComment) -> Self {
        FluentEntry::Comment(comment)
    }
}

impl From<FluentJunk> for FluentEntry {
    fn from(junk: FluentJunk) -> Self {
        FluentEntry::Junk(junk)
    }
}
