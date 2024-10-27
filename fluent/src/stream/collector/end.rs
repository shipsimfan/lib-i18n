use crate::StreamCollector;

impl<'a, 'b> StreamCollector<'a, 'b> {
    /// Finish collecting and return the string
    pub fn end(self) -> &'a str {
        &self.stream.source[self.start_index..self.stream.index]
    }
}
