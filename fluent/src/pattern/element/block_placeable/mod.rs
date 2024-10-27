use crate::FluentInlinePlaceable;

mod deref;
mod display;
mod from;
mod get;
mod new;
mod parse;

/// A placeable that begins on a new line
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentBlockPlaceable {
    /// The placeable itself
    placeable: FluentInlinePlaceable,
}
