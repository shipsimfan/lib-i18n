use crate::fluent::{Blank, BlankInline, FluentAttribute, LineEnd, Parse, Stream};

impl Parse for FluentAttribute {
    fn parse(stream: &mut Stream) -> Option<Self> {
        stream.expect(LineEnd)?;

        stream.step_parse::<Blank>();

        stream.expect('.')?;
        let name = stream.parse()?;

        stream.step_parse::<BlankInline>();
        stream.expect('=')?;
        stream.step_parse::<BlankInline>();

        let pattern = stream.parse()?;

        Some(FluentAttribute { name, pattern })
    }
}
