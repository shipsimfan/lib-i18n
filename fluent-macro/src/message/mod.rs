use fluent::FluentIdentifier;
use format::IncludeFluentFormat;

mod format;

/// A single message which can be displayed in different languages
pub struct IncludeFluentMessage {
    /// The name of this message
    name: FluentIdentifier,

    /// The formats used to display this message in the different languages
    format: Vec<IncludeFluentFormat>,
}
