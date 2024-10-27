use crate::{BlankInline, LineEnd, Parse, Stream};

pub(crate) struct BlankBlock;

impl Parse for BlankBlock {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let mut lines = 0;

        while !stream.empty() {
            if stream
                .step(|stream| {
                    stream.step_parse::<BlankInline>();

                    if stream.parse::<LineEnd>().is_none() {
                        return None;
                    }

                    Some(())
                })
                .is_none()
            {
                break;
            }

            lines += 1;
        }

        if lines == 0 {
            return None;
        }

        Some(BlankBlock)
    }
}
