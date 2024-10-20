use crate::fluent::{FluentPosition, Stream};

impl<'a> Stream<'a> {
    /// Gets the position of the next character
    pub fn position(&self) -> FluentPosition {
        self.position
    }
}
