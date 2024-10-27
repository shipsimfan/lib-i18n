use crate::StreamCollector;

impl<'a, 'b> StreamCollector<'a, 'b> {
    /// Gets the next character from stream, advancing it
    pub(crate) fn next(&mut self) -> Option<char> {
        self.stream.next()
    }
}
