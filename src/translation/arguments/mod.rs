mod argument;
mod generic;

pub use argument::Argument;
pub use generic::GenericArguments;

/// A set of arguments to be passed to a [`Message`](crate::translation::Message)
pub trait Arguments {
    /// Get the value for `key`
    fn get_argument<'a>(&'a self, key: &str) -> Option<&'a Argument>;
}
