use crate::MessageDisplay;

impl<Arguments> core::fmt::Display for MessageDisplay<Arguments> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (self.message)(&self.arguments, f)
    }
}
