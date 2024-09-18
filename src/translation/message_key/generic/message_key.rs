use crate::{
    locale::LanguageTag,
    translation::{Arguments, GenericMessageKey, Message, MessageKey},
};

impl<A: Arguments> MessageKey for GenericMessageKey<A> {
    type Arguments = A;

    fn get_message<'a>(&'a self, _: &LanguageTag) -> &'a Message<A> {
        todo!()
    }
}
