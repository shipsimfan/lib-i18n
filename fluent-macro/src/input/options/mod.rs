use proc_macro_util::Span;

mod get;
mod parse;

mod option;

use option::IncludeFluentOption;

/// The options requested in the `include_fluent!` macro
pub struct IncludeFluentOptions {
    /// The requested fallback language
    fallback: Option<(String, Span)>,
}
