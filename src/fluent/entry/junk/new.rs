use crate::fluent::{FluentJunk, FluentPosition};
use alloc::string::String;

impl FluentJunk {
    /// Creates a new [`FluentJunk`] entry
    pub fn new<P: Into<FluentPosition>, S: Into<String>>(position: P, content: S) -> Self {
        let position = position.into();
        let content = content.into();

        FluentJunk { position, content }
    }
}
