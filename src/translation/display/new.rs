use crate::translation::{Message, MessageDisplay};

impl<'a, A> MessageDisplay<'a, A> {
    /// Creates a new [`MessageDisplay`] for `message` with `arguments`
    pub(in crate::translation) const fn new(message: &'a Message<A>, arguments: A) -> Self {
        MessageDisplay { message, arguments }
    }
}
