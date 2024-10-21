use crate::fluent::{FluentEntry, FluentResource};

impl FluentResource {
    /// Pushes `entry` onto the end of this resource
    pub fn push<T: Into<FluentEntry>>(&mut self, entry: T) {
        let entry = entry.into();

        if let Some(last) = self.entries.last() {
            assert!(last.position() < entry.position());
        }

        self.entries.push(entry);
    }
}
