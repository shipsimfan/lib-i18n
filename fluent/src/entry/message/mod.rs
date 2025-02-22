use crate::{FluentAttribute, FluentIdentifier, FluentPattern};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

mod display;
mod get;
mod new;
mod parse;

/// A single fluent message
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentMessage {
    /// The name of the message
    name: FluentIdentifier,

    /// The pattern describing this message
    pattern: Option<FluentPattern>,

    /// Child attributes to a message
    attributes: Vec<FluentAttribute>,
}
