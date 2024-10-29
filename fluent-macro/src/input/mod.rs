use proc_macro_util::Span;

mod options;

mod deref;
mod get;
mod parse;

pub use options::IncludeFluentOptions;

/// The input to the `include_fluent!` macro
pub struct IncludeFluentInput {
    /// The path to the file or directory containing fluent resources
    path: (String, Span),

    /// The requested options
    options: IncludeFluentOptions,
}
