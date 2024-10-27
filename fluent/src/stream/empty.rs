use crate::Stream;

impl<'a> Stream<'a> {
    /// Is the stream empty?
    pub(crate) const fn empty(&self) -> bool {
        self.index >= self.source.len()
    }
}
