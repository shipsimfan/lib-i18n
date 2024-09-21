use crate::{
    locale::LanguageTag,
    translation::{message_key::Messages, Message, MessageKey},
};

impl<'a, A> MessageKey<'a, A> {
    /// Creates a new [`MessageKey`] from `messages` where the first message is used as the final fallback
    pub const fn new(messages: &'a [(&'a LanguageTag<'a>, Message<A>)]) -> Self {
        MessageKey {
            messages: Messages::Borrowed(messages),
        }
    }

    /// Creates a new [`MessageKey`] from `messages` where the first message is used as the final fallback
    #[cfg(feature = "alloc")]
    pub const fn new_owned(messages: alloc::vec::Vec<(&'a LanguageTag<'a>, Message<A>)>) -> Self {
        MessageKey {
            messages: Messages::Owned(messages),
        }
    }
}
