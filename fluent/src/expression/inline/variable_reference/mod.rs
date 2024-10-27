use crate::{FluentIdentifier, FluentPosition};

mod deref;
mod display;
mod get;
mod new;
mod parse;

/// A referenece to a variable provided by the user of the message
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentVariableReference {
    /// The position of the start of the variable reference
    position: FluentPosition,

    /// The name of the variable
    name: FluentIdentifier,
}
