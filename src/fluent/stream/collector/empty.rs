use crate::fluent::StreamCollector;

impl<'a, 'b> StreamCollector<'a, 'b> {
    /// Is the stream empty?
    pub(in crate::fluent) const fn empty(&self) -> bool {
        self.stream.empty()
    }
}
