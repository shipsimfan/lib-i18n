use crate::fluent::{FluentAttribute, FluentIdentifier, FluentPattern, FluentPosition};

mod display;
mod get;
mod new;
mod parse;

/// A single fluent term
#[derive(Debug, Clone, PartialEq, Eq)]
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
