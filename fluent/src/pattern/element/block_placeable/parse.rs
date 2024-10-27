use crate::{BlankBlock, BlankInline, FluentBlockPlaceable, Parse, Stream};

impl Parse for FluentBlockPlaceable {
    fn parse(stream: &mut Stream) -> Option<Self> {
        stream.step_parse::<BlankBlock>()?;
        stream.step_parse::<BlankInline>();

        let placeable = stream.parse()?;

        Some(FluentBlockPlaceable { placeable })
    }
}
