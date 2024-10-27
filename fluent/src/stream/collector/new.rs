use crate::{Stream, StreamCollector};

impl<'a, 'b> StreamCollector<'a, 'b> {
    /// Creates a new [`StreamCollector`] over `stream`
    pub(in crate::stream) fn new(stream: &'b mut Stream<'a>) -> Self {
        let start_index = stream.index;

        StreamCollector {
            stream,
            start_index,
        }
    }
}
