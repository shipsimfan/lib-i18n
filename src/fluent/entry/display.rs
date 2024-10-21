use crate::fluent::FluentEntry;

impl core::fmt::Display for FluentEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FluentEntry::Comment(comment) => comment.fmt(f),
            FluentEntry::Junk(junk) => junk.fmt(f),
        }
    }
}
