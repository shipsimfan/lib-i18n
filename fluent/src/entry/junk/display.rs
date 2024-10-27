use crate::FluentJunk;

impl core::fmt::Display for FluentJunk {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.content.fmt(f)
    }
}
