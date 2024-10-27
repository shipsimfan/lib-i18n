use crate::{FluentJunk, Parse, Stream, StreamCollector};
use alloc::string::ToString;

fn parse_line(collector: &mut StreamCollector) {
    while let Some(c) = collector.next() {
        if c == '\n' {
            break;
        }
    }
}

impl Parse for FluentJunk {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let position = stream.position();
        let mut collector = stream.begin_collecting();

        // Parse through first line
        parse_line(&mut collector);

        // Parse lines until a "trigger" character is found
        loop {
            match collector.peek() {
                None | Some('#') | Some('-') => break,
                Some(x) if x.is_ascii_alphabetic() => break,
                _ => {}
            }

            parse_line(&mut collector);
        }

        Some(FluentJunk {
            position,
            content: collector.end().to_string(),
        })
    }
}
