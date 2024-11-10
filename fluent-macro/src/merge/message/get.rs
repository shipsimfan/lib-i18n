use crate::{IncludeFluentVariable, MergedMessage};
use i18n_fluent::{FluentPattern, FluentResource};
use i18n_locale::LanguageTag;
use std::collections::HashMap;

impl<'a> MergedMessage<'a> {
    /// Gets the messages for each language
    pub fn languages(
        &self,
    ) -> &HashMap<&'a LanguageTag<'static>, (&'a FluentPattern, &'a FluentResource)> {
        &self.languages
    }

    /// Gets the variables required for this message
    pub fn variables(&self) -> &HashMap<String, IncludeFluentVariable> {
        &self.variables
    }
}
