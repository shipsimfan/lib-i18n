use crate::{message_key::Messages, Message};
use locale::LanguageTag;

impl<'a, 'b, A> IntoIterator for &'b Messages<'a, A> {
    type IntoIter = core::slice::Iter<'b, (&'a LanguageTag<'a>, Message<A>)>;
    type Item = &'b (&'a LanguageTag<'a>, Message<A>);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
