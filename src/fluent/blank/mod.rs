use crate::fluent::{LineEnd, Parse, Stream};

mod block;
mod inline;

pub(in crate::fluent) use block::BlankBlock;
pub(in crate::fluent) use inline::BlankInline;

pub(in crate::fluent) struct Blank;

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
