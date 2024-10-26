use crate::fluent::{
    blank::{BlankBlock, BlankInline},
    FluentBlockText, Parse, Stream,
};

impl Parse for FluentBlockText {
    fn parse(stream: &mut Stream) -> Option<Self> {
        stream.parse::<BlankBlock>()?;
        stream.parse::<BlankInline>()?;

        let position = stream.position();
        let mut collector = stream.begin_collecting();

        match collector.next() {
            Some('[') | Some('*') | Some('.') | Some('{') | Some('}') | Some('\n') | None => {
                return None
            }
            _ => {}
        }

        while let Some(c) = collector.peek() {
            if c == '{' || c == '}' || c == '\n' {
                break;
            }

            collector.next();
        }

        let content = collector.end().to_owned();

        Some(FluentBlockText { position, content })
    }
}
