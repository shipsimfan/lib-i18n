use crate::translation::{Arguments, Message};

/// A message provided with arguments which can be [`Display`](std::fmt::Display)ed
pub struct MessageDisplay<'a, A: Arguments> {
    /// The message to display
    message: &'a Message<A>,

    /// The arguments to use
    arguments: A,
}
