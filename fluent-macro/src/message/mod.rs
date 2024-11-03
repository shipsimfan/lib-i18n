mod format;
mod render;

pub use format::IncludeFluentFormat;

/// A single message which can be displayed in different languages
pub struct IncludeFluentMessage {
    /// The name of this message
    name: String,

    /// The formats used to display this message in the different languages
    format: Vec<IncludeFluentFormat>,
}
