use crate::{FluentIdentifier, FluentPattern, FluentPosition};

mod display;
mod get;
mod new;
mod parse;

/// A child message to a full entry message or term
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentAttribute {
    /// The position of the start of this attribute
    position: FluentPosition,

    /// The name of the attribute, without leading '.'
    name: FluentIdentifier,

    /// The pattern defining this attribute
    pattern: FluentPattern,
}
