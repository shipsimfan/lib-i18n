use crate::{FluentVariableReference, Parse, Stream};

impl Parse for FluentVariableReference {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let position = stream.position();

        stream.expect('$')?;

        let name = stream.parse()?;

        Some(FluentVariableReference { position, name })
    }
}
