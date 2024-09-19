use crate::translation::MessageDisplay;

mod new;

/// A message translated into a specific language
pub struct Message<A> {
    /// The function which displays this message
    pub(in crate::translation) display: fn(&A, &mut std::fmt::Formatter) -> std::fmt::Result,
}

impl<A> Message<A> {
    /// Gets an item which can display this message with `arguments`
    pub const fn display<'a>(&'a self, arguments: &'a A) -> MessageDisplay<'a, A> {
        MessageDisplay::new(self, arguments)
    }
}
