use crate::Message;
use locale::LanguageTag;

mod deref;
mod iter;

pub(super) enum Messages<'a, A> {
    Borrowed(&'a [(&'a LanguageTag<'a>, Message<A>)]),
    #[cfg(feature = "alloc")]
    Owned(alloc::vec::Vec<(&'a LanguageTag<'a>, Message<A>)>),
}
