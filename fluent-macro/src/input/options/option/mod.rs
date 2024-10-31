use locale::LanguageTag;
use proc_macro_util::Span;

mod parse;

/// An option provided to the `include_fluent!` macro
pub(in crate::input) enum IncludeFluentOption {
    /// A specified fallback language
    Fallback {
        /// The request fallback value
        value: LanguageTag<'static>,

        /// The [`Span`] of the requested value
        value_span: Span,

        /// The [`Span`] of the name of this option
        name_span: Span,
    },
}
