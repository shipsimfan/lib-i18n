use crate::fluent::FluentEntry;
use alloc::vec::Vec;

mod display;
mod get;
mod new;
mod parse;
mod push;

/// A parsed fluent file
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FluentResource {
    /// The entries of a fluent file
    entries: Vec<FluentEntry>,
}
