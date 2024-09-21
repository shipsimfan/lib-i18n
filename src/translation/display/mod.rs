use crate::translation::Message;

mod display;
mod r#macro;
mod new;

/// A message provided with arguments which can be [`Display`](core::fmt::Display)ed
pub struct MessageDisplay<'a, A> {
    /// The message to display
    message: &'a Message<A>,

    /// The arguments to use
    arguments: &'a A,
}
