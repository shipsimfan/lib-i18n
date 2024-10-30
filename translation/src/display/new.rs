use crate::{Message, MessageDisplay};

impl<'a, A> MessageDisplay<'a, A> {
    /// Creates a new [`MessageDisplay`] for `message` with `arguments`
    pub(crate) const fn new(message: &'a Message<A>, arguments: &'a A) -> Self {
        MessageDisplay { message, arguments }
    }
}
