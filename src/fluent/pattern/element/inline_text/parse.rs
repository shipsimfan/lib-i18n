use crate::fluent::{FluentInlineText, Parse, Stream};

fn is_valid_char(c: char) -> bool {
    c == '{' || c == '}' || c == '\n'
}

impl Parse for FluentInlineText {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let position = stream.position();
        let mut collector = stream.begin_collecting();

        let first = collector.next()?;
        if !is_valid_char(first) {
            return None;
        }

        while let Some(c) = collector.peek() {
            if !is_valid_char(c) {
                break;
            }

            collector.next();
        }

        let content = collector.end().to_owned();
        Some(FluentInlineText { position, content })
    }
}
