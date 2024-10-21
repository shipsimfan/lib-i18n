use crate::fluent::{FluentEntry, FluentJunk};

impl From<FluentJunk> for FluentEntry {
    fn from(junk: FluentJunk) -> Self {
        FluentEntry::Junk(junk)
    }
}
