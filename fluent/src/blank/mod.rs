use crate::{LineEnd, Parse, Stream};

mod block;
mod inline;

pub(crate) use block::BlankBlock;
pub(crate) use inline::BlankInline;

pub(crate) struct Blank;

impl Parse for Blank {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let mut count = 0;
        loop {
            if stream.empty()
                || (stream.step_parse::<LineEnd>().is_none()
                    && stream.step_parse::<BlankInline>().is_none())
            {
                break;
            }

            count += 1;
        }

        if count == 0 {
            None
        } else {
            Some(Blank)
        }
    }
}
