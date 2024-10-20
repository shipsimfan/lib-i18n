use crate::fluent::{FluentPosition, Stream};

impl<'a> Stream<'a> {
    /// Creates a new [`Stream`] over `source`
    pub(in crate::fluent) fn new(source: &'a str) -> Self {
        Stream {
            source,
            characters: source.chars(),
            index: 0,
            position: FluentPosition::new(),
        }
    }
}
