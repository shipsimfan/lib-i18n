use crate::{Parse, Stream};

impl<'a> Stream<'a> {
    /// Parses `val` from the stream, returning [`Some`] only if the parsed value is equal `val`
    pub fn expect<T: Parse + PartialEq>(&mut self, val: T) -> Option<T> {
        let parsed = self.parse()?;
        if parsed != val {
            return None;
        }

        Some(parsed)
    }
}
