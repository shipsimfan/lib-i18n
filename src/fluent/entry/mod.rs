mod comment;
mod junk;
mod message;
mod term;

mod display;
mod from;
mod get;
mod parse;

pub use comment::FluentComment;
pub use junk::FluentJunk;
pub use message::FluentMessage;
pub use term::FluentTerm;

/// An entry in a fluent file
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FluentEntry {
    /// A message which describes a usuable translation
    Message(FluentMessage),

    /// A term which can be included in a message or another term
    Term(FluentTerm),

    /// A comment describing the contents of a fluent file
    Comment(FluentComment),

    /// An entry that was not parsable
    Junk(FluentJunk),
}
