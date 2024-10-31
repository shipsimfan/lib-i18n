use crate::IncludeFluentOptions;
use locale::LanguageTag;
use proc_macro_util::Span;

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

    /// Gets the [`Span`] of the request fallback language
    pub fn fallback_span(&self) -> Span {
        match &self.fallback {
            Some((_, span)) => *span,
            None => Span::call_site(),
        }
    }
}
