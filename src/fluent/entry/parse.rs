use crate::fluent::{FluentEntry, Parse, Stream};

impl Parse for FluentEntry {
    fn parse(stream: &mut Stream) -> Option<Self> {
        match stream.peek() {
            Some('#') => {
                if let Some(comment) = stream.step_parse() {
                    return Some(FluentEntry::Comment(comment));
                }
            }
            _ => {}
        }

        Some(FluentEntry::Junk(stream.parse().unwrap()))
    }
}
