use crate::{IncludeFluentInput, IncludeFluentOptions};

impl IncludeFluentInput {
    /// Gets the path to the file or directory containing fluent resources
    pub fn path(&self) -> &str {
        &self.path.0
    }

    /// Gets the options specified in the macro
    pub const fn options(&self) -> &IncludeFluentOptions {
        &self.options
    }
}
