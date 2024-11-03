use std::borrow::Borrow;

use i18n_locale::LanguageTag;

use crate::SupportedLanguage;

impl Borrow<LanguageTag<'static>> for SupportedLanguage {
    fn borrow(&self) -> &LanguageTag<'static> {
        self.tag()
    }
}
