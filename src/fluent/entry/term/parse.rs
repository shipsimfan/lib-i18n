use crate::fluent::{blank::BlankInline, line_end::LineEnd, FluentTerm, Parse, Stream};

impl Parse for FluentTerm {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let position = stream.position();
        stream.expect('-')?;
        let name = stream.parse()?;

        stream.step_parse::<BlankInline>();
        stream.expect('=')?;
        stream.step_parse::<BlankInline>();

        let pattern = stream.step_parse()?;

        let mut attributes = Vec::new();
        while let Some(attribute) = stream.step_parse() {
            attributes.push(attribute);
        }

        stream.expect(LineEnd)?;

        Some(FluentTerm {
            position,
            name,
            pattern,
            attributes,
        })
    }
}
