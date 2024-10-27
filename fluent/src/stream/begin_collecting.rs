use crate::{Stream, StreamCollector};

impl<'a> Stream<'a> {
    /// Begins collecting characters from this stream
    pub(crate) fn begin_collecting<'b>(&'b mut self) -> StreamCollector<'a, 'b> {
        StreamCollector::new(self)
    }
}
