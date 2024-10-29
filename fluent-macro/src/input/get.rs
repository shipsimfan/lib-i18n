use crate::{IncludeFluentInput, IncludeFluentOptions};
use proc_macro_util::Span;

impl IncludeFluentInput {
    /// Gets the path to the file or directory containing fluent resources
    pub fn path(&self) -> &str {
        &self.path.0
    }

    /// Gets the [`Span`] of the path
    pub fn path_span(&self) -> Span {
        self.path.1
    }

    /// Gets the options specified in the macro
    pub const fn options(&self) -> &IncludeFluentOptions {
        &self.options
    }
}
