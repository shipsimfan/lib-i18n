mod inline;

mod display;
mod from;
mod get;
mod new;
mod parse;

pub use inline::{FluentInlineExpression, FluentVariableReference};

/// An expression which describes an element to be inserted into a pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FluentExpression {
    /// The expression is an inline expression
    Inline(FluentInlineExpression),
}
