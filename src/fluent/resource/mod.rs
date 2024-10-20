use crate::fluent::FluentJunk;

mod get;
mod parse;

/// A parsed fluent file
#[derive(Debug, Clone)]
pub struct FluentResource {
    /// The junk from the file that couldn't be parsed
    junk: Vec<FluentJunk>,
}
