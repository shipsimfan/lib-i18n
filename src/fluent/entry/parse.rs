use crate::fluent::{BlankBlock, FluentEntry, FluentMessage, Parse, Stream};

impl Parse for FluentEntry {
    fn parse(stream: &mut Stream) -> Option<Self> {
        match stream.peek() {
            Some('#') => {
                if let Some(comment) = stream.step_parse() {
                    return Some(FluentEntry::Comment(comment));
                }
            }
            Some(' ') | Some('\n') | None => {
                if stream.step_parse::<BlankBlock>().is_some() {
                    return None;
                }
            }
            Some(x) if x.is_ascii_alphabetic() => {
                if let Some(message) = stream.step_parse::<FluentMessage>() {
                    return Some(FluentEntry::Message(message));
                }
            }
            _ => {}
        }

        Some(FluentEntry::Junk(stream.parse().unwrap()))
    }
}
