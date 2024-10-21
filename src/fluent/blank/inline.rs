use crate::fluent::{Parse, Stream};

pub(in crate::fluent) struct BlankInline;

impl Parse for BlankInline {
    fn parse(stream: &mut Stream) -> Option<Self> {
        match stream.next() {
            Some(' ') => {}
            _ => return None,
        }

        while let Some(c) = stream.peek() {
            if c != ' ' {
                break;
            }

            stream.next();
        }

        Some(BlankInline)
    }
}
