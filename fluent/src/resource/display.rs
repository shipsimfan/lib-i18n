use crate::FluentResource;

impl core::fmt::Display for FluentResource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for entry in &self.entries {
            entry.fmt(f)?;
        }

        Ok(())
    }
}
