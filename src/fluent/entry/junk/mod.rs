use crate::fluent::FluentPosition;
use alloc::string::String;

mod display;
mod get;
mod new;
mod parse;

/// Parts of a fluent file that were not able to be parsed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluentJunk {
    /// The position of the start of this junk
    position: FluentPosition,

    /// The content of the junk
    content: String,
}
