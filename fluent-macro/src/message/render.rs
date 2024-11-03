use crate::{IncludeFluentFormat, IncludeFluentMessage, MergedMessage, SupportedLanguage};
use i18n_fluent::FluentIdentifier;
use proc_macro_util::{tokens::Identifier, Result};
use std::collections::HashSet;

impl IncludeFluentMessage {
    /// Renders a module into message keys
    pub fn render(
        message: &MergedMessage,
        name: &FluentIdentifier,
        supported_languages: &mut HashSet<SupportedLanguage>,
    ) -> Result<Self> {
        let mut format = Vec::new();
        for (language, (pattern, resource)) in message.languages() {
            let language = if let Some(supported_language) = supported_languages.get(*language) {
                supported_language.identifier().clone()
            } else {
                let supported_language = SupportedLanguage::new(language);
                let identifier = supported_language.identifier().clone();
                supported_languages.insert(supported_language);
                identifier
            };
            format.push(IncludeFluentFormat::render(language, pattern, resource)?);
        }

        Ok(IncludeFluentMessage {
            name: Identifier::new(&name.to_string()),
            format,
        })
    }
}
