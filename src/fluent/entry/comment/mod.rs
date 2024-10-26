use crate::fluent::FluentPosition;
use alloc::string::String;

mod display;
mod get;
mod new;
mod parse;

/// A comment in a fluent file
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentComment {
    /// The position of the start of this comment
    position: FluentPosition,

    /// The number of `#` characters prefixing this comment
    hashes: u8,

    /// The content of the comment
    content: String,
}
