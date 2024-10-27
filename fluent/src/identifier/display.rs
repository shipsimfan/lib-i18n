use crate::FluentIdentifier;

impl core::fmt::Display for FluentIdentifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.content.fmt(f)
    }
}
