use crate::SupportedLanguage;
use i18n_locale::LanguageTag;
use proc_macro_util::tokens::Identifier;

impl SupportedLanguage {
    /// Gets the identifier for this language
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    /// Gets the parsed language tag
    pub fn tag(&self) -> &LanguageTag<'static> {
        &self.tag
    }
}
