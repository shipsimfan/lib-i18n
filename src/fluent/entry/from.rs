use crate::fluent::{FluentComment, FluentEntry, FluentJunk};

impl From<FluentJunk> for FluentEntry {
    fn from(junk: FluentJunk) -> Self {
        FluentEntry::Junk(junk)
    }
}

impl From<FluentComment> for FluentEntry {
    fn from(comment: FluentComment) -> Self {
        FluentEntry::Comment(comment)
    }
}
