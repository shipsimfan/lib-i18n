use crate::fluent::{FluentIdentifier, Parse, Stream};

impl Parse for FluentIdentifier {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let position = stream.position();
        let mut collector = stream.begin_collecting();

        match collector.next() {
            Some(x) if x.is_ascii_alphabetic() => {}
            _ => return None,
        }

        while let Some(c) = collector.peek() {
            if !c.is_ascii_alphanumeric() && c != '_' && c != '-' {
                break;
            }

            collector.next();
        }

        let content = collector.end().to_owned();

        Some(FluentIdentifier { position, content })
    }
}
