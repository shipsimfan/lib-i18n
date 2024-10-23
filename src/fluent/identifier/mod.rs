use crate::fluent::FluentPosition;

mod deref;
mod display;
mod get;
mod new;
mod parse;

/// An identifier of an element
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentIdentifier {
    /// The position of the start of this identifier
    position: FluentPosition,

    // The content of the identifier
    content: String,
}
