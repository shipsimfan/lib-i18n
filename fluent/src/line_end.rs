use crate::{Parse, Stream};

#[derive(PartialEq, Eq)]
pub(crate) struct LineEnd;

impl Parse for LineEnd {
    fn parse(stream: &mut Stream) -> Option<Self> {
        if let Some('\r') = stream.peek() {
            if let Some('\n') = stream.peek_n(2) {
                stream.skip(2);
                return Some(LineEnd);
            }
        }

        match stream.peek() {
            Some('\n') | None => {
                stream.next();
                Some(LineEnd)
            }
            _ => None,
        }
    }
}
