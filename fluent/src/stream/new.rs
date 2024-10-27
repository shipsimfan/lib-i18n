use crate::{FluentPosition, Stream};

impl<'a> Stream<'a> {
    /// Creates a new [`Stream`] over `source`
    pub(crate) fn new(source: &'a str) -> Self {
        Stream {
            source,
            characters: source.chars(),
            index: 0,
            position: FluentPosition::default(),
        }
    }
}
