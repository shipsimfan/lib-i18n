use crate::FluentInlinePlaceable;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

mod display;
mod from;
mod get;
mod new;
mod parse;

/// An expression that can fit on a single line
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FluentInlineExpression {
    /// An inline placeable inside this expression
    InlinePlaceable(Box<FluentInlinePlaceable>),
}
