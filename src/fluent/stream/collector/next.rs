use crate::fluent::StreamCollector;

impl<'a, 'b> StreamCollector<'a, 'b> {
    /// Gets the next character from stream, advancing it
    pub(in crate::fluent) fn next(&mut self) -> Option<char> {
        self.stream.next()
    }
}
