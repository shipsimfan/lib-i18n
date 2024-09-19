use crate::translation::Message;

impl<A> Message<A> {
    /// Creates a new [`Message`] from `display` which prints this message given some arguments
    pub const fn new(display: fn(&A, &mut std::fmt::Formatter) -> std::fmt::Result) -> Self {
        Message { display }
    }
}
