use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// An insert to be replaced with a variable
pub struct IncludeFluentFormatInsert {
    /// The name of the variable to use
    name: Identifier,
}
