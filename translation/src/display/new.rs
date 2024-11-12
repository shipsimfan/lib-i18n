use crate::{Message, MessageDisplay};

impl<Arguments> MessageDisplay<Arguments> {
    /// Creates a new [`MessageDisplay`] for `message` with `arguments`
    pub(crate) const fn new(message: Message<Arguments>, arguments: Arguments) -> Self {
        MessageDisplay { message, arguments }
    }
}
