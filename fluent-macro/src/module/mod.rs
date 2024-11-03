use crate::IncludeFluentMessage;
use proc_macro_util::tokens::Identifier;

mod render;
mod to_tokens;

/// A module of message keys and sub modules
pub struct IncludeFluentModule {
    /// How many modules are between this and the supported languages
    depth: usize,

    /// The submodules containing more messages
    sub_modules: Vec<(Identifier, IncludeFluentModule)>,

    /// The messages contained in this module
    messages: Vec<IncludeFluentMessage>,
}
