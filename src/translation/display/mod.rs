use crate::translation::Message;

mod display;
mod new;

/// A message provided with arguments which can be [`Display`](std::fmt::Display)ed
pub struct MessageDisplay<'a, A> {
    /// The message to display
    message: &'a Message<A>,

    /// The arguments to use
    arguments: &'a A,
}
