use crate::MessageDisplay;

mod r#macro;
mod new;

/// A message translated into a specific language
pub struct Message<A> {
    /// The function which displays this message
    pub(crate) display: fn(&A, &mut core::fmt::Formatter) -> core::fmt::Result,
}

impl<A> Message<A> {
    /// Gets an item which can display this message with `arguments`
    pub const fn display<'a>(&'a self, arguments: &'a A) -> MessageDisplay<'a, A> {
        MessageDisplay::new(self, arguments)
    }
}
