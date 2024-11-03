use crate::{IncludeFluentFormat, IncludeFluentMessage, MergedMessage};
use fluent::FluentIdentifier;
use locale::LanguageTag;
use proc_macro_util::Result;
use std::collections::HashSet;

impl IncludeFluentMessage {
    /// Renders a module into message keys
    pub fn render(
        message: &MergedMessage,
        name: &FluentIdentifier,
        supported_languages: &mut HashSet<LanguageTag<'static>>,
    ) -> Result<Self> {
        let mut format = Vec::new();
        for (language, (pattern, resource)) in message.languages() {
            let language = supported_languages
                .replace(language.static_clone())
                .unwrap_or(language.static_clone());
            format.push(IncludeFluentFormat::render(language, pattern, resource)?);
        }

        Ok(IncludeFluentMessage {
            name: name.to_string(),
            format,
        })
    }
}
