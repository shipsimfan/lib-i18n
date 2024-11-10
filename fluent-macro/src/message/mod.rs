use proc_macro_util::{tokens::Identifier, Token};

mod format;
mod variable;

mod render;
mod to_tokens;

pub use format::IncludeFluentFormat;
pub use variable::IncludeFluentVariable;

/// A single message which can be displayed in different languages
pub struct IncludeFluentMessage {
    /// The name of this message
    name: Identifier,

    /// The variables needed for this message
    variables: Vec<(Identifier, Token![:], IncludeFluentVariable)>,

    /// The formats used to display this message in the different languages
    format: Vec<IncludeFluentFormat>,
}
