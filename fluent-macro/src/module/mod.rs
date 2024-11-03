use crate::IncludeFluentMessage;

mod render;
mod to_tokens;

/// A module of message keys and sub modules
pub struct IncludeFluentModule {
    /// The submodules containing more messages
    sub_modules: Vec<(String, IncludeFluentModule)>,

    /// The messages contained in this module
    messages: Vec<IncludeFluentMessage>,
}
