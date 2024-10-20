use crate::fluent::{FluentJunk, FluentResource};

impl FluentResource {
    /// Gets the junk lines in the file
    pub fn junk(&self) -> &[FluentJunk] {
        &self.junk
    }
}
