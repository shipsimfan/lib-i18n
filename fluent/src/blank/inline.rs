use crate::{Parse, Stream};

pub(crate) struct BlankInline;

impl Parse for BlankInline {
    fn parse(stream: &mut Stream) -> Option<Self> {
        stream.expect(' ')?;

        while let Some(c) = stream.peek() {
            if c != ' ' {
                break;
            }

            stream.next();
        }

        Some(BlankInline)
    }
}
