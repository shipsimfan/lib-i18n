use crate::fluent::{FluentComment, FluentEntry, FluentJunk, FluentMessage};

impl From<FluentMessage> for FluentEntry {
    fn from(message: FluentMessage) -> Self {
        FluentEntry::Message(message)
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
