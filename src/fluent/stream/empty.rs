use crate::fluent::Stream;

impl<'a> Stream<'a> {
    /// Is the stream empty?
    pub(in crate::fluent) const fn empty(&self) -> bool {
        self.index >= self.source.len()
    }
}
