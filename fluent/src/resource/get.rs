use crate::{FluentEntry, FluentResource};

impl FluentResource {
    /// Gets the entries of the file
    pub fn entries(&self) -> &[FluentEntry] {
        &self.entries
    }
}
