use crate::translation::MessageDisplay;

impl<'a, A> std::fmt::Display for MessageDisplay<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.message.display)(&self.arguments, f)
    }
}
