use crate::fluent::{FluentIdentifier, FluentPattern};

mod display;
mod get;
mod new;
mod parse;

/// A child message to a full entry message or term
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluentAttribute {
    /// The name of the attribute, without leading '.'
    name: FluentIdentifier,

    /// The pattern defining this attribute
    pattern: FluentPattern,
}
