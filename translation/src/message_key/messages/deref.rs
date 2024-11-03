use crate::{locale::LanguageTag, message_key::Messages, Message};

impl<'a, A> core::ops::Deref for Messages<'a, A> {
    type Target = [(&'a LanguageTag<'a>, Message<A>)];

    fn deref(&self) -> &Self::Target {
        match self {
            Messages::Borrowed(messages) => messages,
            #[cfg(feature = "alloc")]
            Messages::Owned(messages) => messages,
        }
    }
}
