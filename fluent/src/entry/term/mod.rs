use crate::{FluentAttribute, FluentIdentifier, FluentPattern, FluentPosition};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

mod display;
mod get;
mod new;
mod parse;

/// A single fluent term
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentTerm {
    /// The position of the start of this term
    position: FluentPosition,

    /// The name of the term
    name: FluentIdentifier,

    /// The pattern describing this term
    pattern: FluentPattern,

    /// Child attributes to this term
    attributes: Vec<FluentAttribute>,
}
