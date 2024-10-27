use crate::FluentPosition;

#[cfg(not(feature = "std"))]
use alloc::string::String;

mod deref;
mod display;
mod get;
mod new;
mod parse;

/// A text element that starts on a different line
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentBlockText {
    /// The position of the start of the text
    position: FluentPosition,

    /// The content of the text
    content: String,
}
