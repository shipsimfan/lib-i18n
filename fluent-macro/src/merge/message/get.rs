use crate::MergedMessage;
use fluent::{FluentPattern, FluentResource};
use locale::LanguageTag;
use std::collections::HashMap;

impl<'a> MergedMessage<'a> {
    /// Gets the messages for each language
    pub fn languages(
        &self,
    ) -> &HashMap<&'a LanguageTag<'static>, (&'a FluentPattern, &'a FluentResource)> {
        &self.languages
    }
}
