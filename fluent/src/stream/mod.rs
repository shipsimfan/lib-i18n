use crate::FluentPosition;
use core::str::Chars;

mod collector;

mod begin_collecting;
mod empty;
mod expect;
mod new;
mod next;
mod parse;
mod peek;
mod position;
mod skip;
mod step;

pub use collector::StreamCollector;

/// A stream of characters
#[derive(Clone)]
pub(crate) struct Stream<'a> {
    /// The source being parsed
    source: &'a str,

    /// The stream of characters
    characters: Chars<'a>,

    /// The index of the next character
    index: usize,

    /// The position of the next character
    position: FluentPosition,
}
