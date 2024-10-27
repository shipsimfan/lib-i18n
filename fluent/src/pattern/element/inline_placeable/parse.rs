use crate::{Blank, FluentInlinePlaceable, Parse, Stream};

impl Parse for FluentInlinePlaceable {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let position = stream.position();

        stream.expect('{')?;
        stream.step_parse::<Blank>()?;

        let expression = stream.parse()?;

        stream.step_parse::<Blank>()?;
        stream.expect('}')?;

        Some(FluentInlinePlaceable {
            position,
            expression,
        })
    }
}
