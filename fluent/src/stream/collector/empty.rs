use crate::StreamCollector;

impl<'a, 'b> StreamCollector<'a, 'b> {
    /// Is the stream empty?
    pub(crate) const fn empty(&self) -> bool {
        self.stream.empty()
    }
}
