use proc_macro_util::tokens::{Identifier, Literal};

mod render;
mod to_tokens;

/// A format string used to display a message in a language
pub struct IncludeFluentFormat {
    /// The language to display
    language: Identifier,

    /// The actual format string
    string: Literal,
    // TODO: Add inserts
}
