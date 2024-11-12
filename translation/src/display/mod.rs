use crate::Message;

mod display;
mod r#macro;
mod new;

/// A [`Message`] provided with `Arguments` which can be [`Display`](core::fmt::Display)ed
pub struct MessageDisplay<Arguments: Sized> {
    /// The message to display
    message: Message<Arguments>,

    /// The arguments to use
    arguments: Arguments,
}
