use crate::FluentEntry;

impl core::fmt::Display for FluentEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FluentEntry::Message(message) => message.fmt(f),
            FluentEntry::Term(term) => term.fmt(f),
            FluentEntry::Comment(comment) => comment.fmt(f),
            FluentEntry::Junk(junk) => junk.fmt(f),
        }
    }
}
