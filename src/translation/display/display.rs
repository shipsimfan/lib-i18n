use crate::translation::MessageDisplay;

impl<'a, A> core::fmt::Display for MessageDisplay<'a, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (self.message.display)(&self.arguments, f)
    }
}
