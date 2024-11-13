use crate::{IncludeFluent, IncludeFluentModule, MergedModule};
use i18n_locale::LanguageTag;
use proc_macro_util::Result;
use std::collections::HashSet;

impl IncludeFluent {
    /// Renders a module into message keys
    pub fn render(fallback_language: &LanguageTag, module: MergedModule) -> Result<Self> {
        let mut supported_languages = HashSet::new();
        let root =
            IncludeFluentModule::render(&module, 0, fallback_language, &mut supported_languages)?;

        Ok(IncludeFluent {
            root,
            supported_languages,
        })
    }
}
