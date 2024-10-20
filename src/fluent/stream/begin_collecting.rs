use crate::fluent::{Stream, StreamCollector};

impl<'a> Stream<'a> {
    /// Begins collecting characters from this stream
    pub(in crate::fluent) fn begin_collecting<'b>(&'b mut self) -> StreamCollector<'a, 'b> {
        StreamCollector::new(self)
    }
}
