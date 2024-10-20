use crate::fluent::FluentPosition;

mod get;
mod parse;

/// Parts of a fluent file that were not able to be parsed
#[derive(Debug, Clone)]
pub struct FluentJunk {
    /// The position of the start of this junk
    position: FluentPosition,

    /// The content of the junk
    content: String,
}
