use std::{borrow::Cow, fmt::Display};

/// A single argument to be passed to a [`Message`](crate::translation::Message)
pub trait Argument: Display {
    /// Gets this argument as a string
    fn as_str(&self) -> Cow<'static, str>;

    /// Gets this argument as a number, if it is one
    fn as_number(&self) -> Option<i128>;
}
