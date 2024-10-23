use crate::fluent::{blank::BlankInline, line_end::LineEnd, FluentMessage, Parse, Stream};

impl Parse for FluentMessage {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let name = stream.parse()?;

        stream.step_parse::<BlankInline>();
        stream.expect('=')?;
        stream.step_parse::<BlankInline>();

        let pattern = stream.step_parse();

        let mut attributes = Vec::new();
        while let Some(attribute) = stream.step_parse() {
            attributes.push(attribute);
        }

        if pattern.is_none() && attributes.len() == 0 {
            return None;
        }

        stream.expect(LineEnd)?;

        Some(FluentMessage {
            name,
            pattern,
            attributes,
        })
    }
}
