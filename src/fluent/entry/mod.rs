mod junk;

mod from;
mod parse;
mod position;

pub use junk::FluentJunk;

/// An entry in a fluent file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluentEntry {
    /// An entry that was not parsable
    Junk(FluentJunk),
}
