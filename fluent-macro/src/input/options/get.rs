use crate::IncludeFluentOptions;
use i18n_locale::LanguageTag;

impl IncludeFluentOptions {
    /// The default fallback language used when none is specified
    pub const DEFAULT_FALLBACK: &LanguageTag<'static> =
        &unsafe { LanguageTag::from_language_unchecked(b"EN") };

    /// Gets the requested fallback language
    pub fn fallback(&self) -> &LanguageTag<'static> {
        match &self.fallback {
            Some((fallback, _)) => fallback,
            None => Self::DEFAULT_FALLBACK,
        }
    }
}
