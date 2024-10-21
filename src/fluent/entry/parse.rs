use crate::fluent::{BlankBlock, FluentEntry, Parse, Stream};

impl Parse for FluentEntry {
    fn parse(stream: &mut Stream) -> Option<Self> {
        match stream.peek() {
            Some('#') => {
                println!("Comment: {}", stream.position().line());
                if let Some(comment) = stream.step_parse() {
                    return Some(FluentEntry::Comment(comment));
                }
            }
            Some(' ') | Some('\n') | None => {
                if stream.step_parse::<BlankBlock>().is_some() {
                    return None;
                }
            }
            _ => {}
        }

        Some(FluentEntry::Junk(stream.parse().unwrap()))
    }
}
