use crate::fluent::FluentInlineText;
use core::ops::Deref;

impl Deref for FluentInlineText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.content()
    }
}
