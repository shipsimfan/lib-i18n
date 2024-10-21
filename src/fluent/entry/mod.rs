mod comment;
mod junk;

mod display;
mod from;
mod get;
mod parse;

pub use comment::FluentComment;
pub use junk::FluentJunk;

/// An entry in a fluent file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluentEntry {
    /// A comment describing the contents of a fluent file
    Comment(FluentComment),

    /// An entry that was not parsable
    Junk(FluentJunk),
}
