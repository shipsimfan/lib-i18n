use crate::FluentBlockText;
use core::ops::Deref;

impl Deref for FluentBlockText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.content()
    }
}

impl AsRef<str> for FluentBlockText {
    fn as_ref(&self) -> &str {
        self.content()
    }
}
