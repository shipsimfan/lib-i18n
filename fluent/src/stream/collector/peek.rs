use crate::StreamCollector;

impl<'a, 'b> StreamCollector<'a, 'b> {
    /// Returns the next character in stream without advacing it
    pub(crate) fn peek(&self) -> Option<char> {
        self.stream.peek()
    }
}
