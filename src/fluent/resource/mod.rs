use crate::fluent::FluentEntry;
use alloc::vec::Vec;

mod get;
mod new;
mod parse;
mod push;

/// A parsed fluent file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluentResource {
    /// The entries of a fluent file
    entries: Vec<FluentEntry>,
}
