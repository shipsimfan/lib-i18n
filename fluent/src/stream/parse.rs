use crate::{Parse, Stream};

impl<'a> Stream<'a> {
    /// Attempts to parse an element from the stream
    pub(crate) fn parse<T: Parse>(&mut self) -> Option<T> {
        T::parse(self)
    }
}
