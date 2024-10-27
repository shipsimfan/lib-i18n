use crate::FluentInlineText;
use core::ops::Deref;

impl Deref for FluentInlineText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.content()
    }
}

impl AsRef<str> for FluentInlineText {
    fn as_ref(&self) -> &str {
        self.content()
    }
}
