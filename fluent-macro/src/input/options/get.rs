use crate::IncludeFluentOptions;
use proc_macro_util::Span;

impl IncludeFluentOptions {
    /// Gets the requested fallback language
    pub fn fallback(&self) -> &str {
        match &self.fallback {
            Some((fallback, _)) => fallback,
            None => "EN",
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
