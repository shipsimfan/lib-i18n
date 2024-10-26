use crate::fluent::{FluentExpression, FluentPosition};

mod deref;
mod display;
mod get;
mod new;
mod parse;

/// An element of a pattern that should be replaced based on some data
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentInlinePlaceable {
    /// The position of the start of this placeable
    position: FluentPosition,

    /// The expression making up this placeable
    expression: FluentExpression,
}
