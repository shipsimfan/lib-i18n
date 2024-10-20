use crate::fluent::{Parse, StreamCollector};

impl<'a, 'b> StreamCollector<'a, 'b> {
    /// Attempts to parse an element from the stream
    pub(in crate::fluent) fn parse<T: Parse>(&mut self) -> Option<T> {
        T::parse(self.stream)
    }
}
