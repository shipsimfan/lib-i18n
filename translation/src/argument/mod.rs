use core::fmt::Display;

/// A single argument to be passed to a [`Message`](crate::translation::Message)
pub trait Argument: Display {
    /// Gets this argument as a string
    #[cfg(feature = "alloc")]
    fn as_str(&self) -> alloc::borrow::Cow<'static, str>;

    /// Gets this argument as a string
    #[cfg(not(feature = "alloc"))]
    fn as_str(&self) -> &'static str;

    /// Gets this argument as a number, if it is one
    fn as_number(&self) -> Option<i128>;
}
