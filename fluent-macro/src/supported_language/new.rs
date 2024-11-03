use crate::SupportedLanguage;
use i18n_locale::LanguageTag;
use proc_macro_util::tokens::Identifier;

impl SupportedLanguage {
    pub fn new(language: &LanguageTag<'static>) -> Self {
        let identifier = Identifier::new(&language.to_string().replace('-', "_"));

        SupportedLanguage {
            identifier,
            tag: language.static_clone(),
        }
    }
}
