use crate::fluent::{FluentJunk, FluentPosition};
use alloc::string::String;

impl FluentJunk {
    /// Creates a new [`FluentJunk`] entry
    pub fn new<P: Into<FluentPosition>>(position: P, content: String) -> Self {
        let position = position.into();

        FluentJunk { position, content }
    }
}
