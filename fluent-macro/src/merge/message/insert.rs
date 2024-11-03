use crate::MergedMessage;
use i18n_fluent::{FluentPattern, FluentResource};
use i18n_locale::LanguageTag;

impl<'a> MergedMessage<'a> {
    /// Inserts a new `pattern` for a given `langauge`, returning true if another pattern was already in place for that language
    pub fn insert(
        &mut self,
        language: &'a LanguageTag<'static>,
        pattern: &'a FluentPattern,
        resource: &'a FluentResource,
    ) -> bool {
        self.languages
            .insert(language, (pattern, resource))
            .is_some()
    }
}
