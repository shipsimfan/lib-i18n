use crate::fluent::FluentIdentifier;
use core::ops::Deref;

impl Deref for FluentIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.content()
    }
}

impl AsRef<str> for FluentIdentifier {
    fn as_ref(&self) -> &str {
        self.content()
    }
}
