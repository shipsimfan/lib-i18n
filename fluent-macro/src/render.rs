use crate::{IncludeFluent, IncludeFluentModule, MergedModule};
use proc_macro_util::Result;
use std::collections::HashSet;

impl IncludeFluent {
    /// Renders a module into message keys
    pub fn render(module: MergedModule) -> Result<Self> {
        let mut supported_languages = HashSet::new();
        let root = IncludeFluentModule::render(&module, &mut supported_languages)?;

        Ok(IncludeFluent {
            root,
            supported_languages,
        })
    }
}
