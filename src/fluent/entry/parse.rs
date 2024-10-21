use crate::fluent::{FluentEntry, Parse, Stream};

impl Parse for FluentEntry {
    fn parse(stream: &mut Stream) -> Option<Self> {
        Some(FluentEntry::Junk(stream.parse().unwrap()))
    }
}
