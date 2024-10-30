use crate::Message;

impl<A> Message<A> {
    /// Creates a new [`Message`] from `display` which prints this message given some arguments
    pub const fn new(display: fn(&A, &mut core::fmt::Formatter) -> core::fmt::Result) -> Self {
        Message { display }
    }
}
