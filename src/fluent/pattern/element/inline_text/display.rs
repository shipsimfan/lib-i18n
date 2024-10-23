use crate::fluent::FluentInlineText;

impl core::fmt::Display for FluentInlineText {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.content.fmt(f)
    }
}
