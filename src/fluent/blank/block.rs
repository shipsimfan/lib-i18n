use crate::fluent::{BlankInline, Parse, Stream};

pub(in crate::fluent) struct BlankBlock;

impl Parse for BlankBlock {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let mut lines = 0;

        while !stream.empty() {
            stream.step_parse::<BlankInline>();

            match stream.peek() {
                Some('\n') | None => {}
                _ => break,
            }

            lines += 1;
            stream.next();
        }

        if lines == 0 {
            return None;
        }

        Some(BlankBlock)
    }
}
