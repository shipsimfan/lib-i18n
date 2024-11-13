use crate::{IncludeFluentFormat, IncludeFluentMessage, MergedMessage, SupportedLanguage};
use i18n_fluent::FluentIdentifier;
use i18n_locale::LanguageTag;
use proc_macro_util::{tokens::Identifier, Error, Result, Token};
use std::collections::HashSet;

impl IncludeFluentMessage {
    /// Renders a module into message keys
    pub fn render(
        message: &MergedMessage,
        name: &FluentIdentifier,
        fallback_language: &LanguageTag,
        supported_languages: &mut HashSet<SupportedLanguage>,
    ) -> Result<Self> {
        let mut fallback = None;
        let mut formats = Vec::new();
        for (language_tag, (pattern, resource)) in message.languages() {
            let language = if let Some(supported_language) = supported_languages.get(*language_tag)
            {
                supported_language.identifier().clone()
            } else {
                let supported_language = SupportedLanguage::new(language_tag);
                let identifier = supported_language.identifier().clone();
                supported_languages.insert(supported_language);
                identifier
            };

            let format = IncludeFluentFormat::render(language, pattern, resource)?;

            if *language_tag == fallback_language {
                fallback = Some(format);
            } else {
                formats.push(format);
            }
        }

        let fallback = fallback.ok_or(Error::new(format_args!(
            "missing fallback langauge (\"{}\") for \"{}\" message",
            fallback_language, name
        )))?;

        let mut variables = Vec::new();
        for (variable_name, variable_type) in message.variables() {
            variables.push((
                Token![pub](),
                Identifier::new(&variable_name),
                Token![:](),
                *variable_type,
            ));
        }

        Ok(IncludeFluentMessage {
            name: Identifier::new(&name.to_string()),
            fallback,
            variables,
            formats,
        })
    }
}
