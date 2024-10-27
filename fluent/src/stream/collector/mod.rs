use crate::Stream;

mod end;
mod new;
mod next;
mod peek;

/// Collects a string from the stream until this is dropped
pub struct StreamCollector<'a, 'b> {
    /// The source of characters
    stream: &'b mut Stream<'a>,

    /// The index of the first character
    start_index: usize,
}
