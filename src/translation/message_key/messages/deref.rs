use crate::{
    locale::LanguageTag,
    translation::{message_key::Messages, Message},
};

impl<'a, A> std::ops::Deref for Messages<'a, A> {
    type Target = [(&'a LanguageTag<'a>, Message<A>)];

    fn deref(&self) -> &Self::Target {
        match self {
            Messages::Borrowed(messages) => messages,
            Messages::Owned(messages) => messages,
        }
    }
}
