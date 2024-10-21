use crate::fluent::StreamCollector;

impl<'a, 'b> StreamCollector<'a, 'b> {
    /// Returns the next character in stream without advacing it
    pub(in crate::fluent) fn peek(&self) -> Option<char> {
        self.stream.peek()
    }
}
