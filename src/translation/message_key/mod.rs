use crate::{
    locale::LanguageTag,
    translation::{Arguments, Message},
};

mod generic;

pub use generic::GenericMessageKey;

/// A set of [`Message`]s with the same meaning in multiple languages
pub trait MessageKey {
    /// The arguments that are to be passed to any [`Message`]
    type Arguments: Arguments;

    /// Get this message translated to `language`
    fn get_message<'a>(&'a self, language: &LanguageTag) -> &'a Message<Self::Arguments>;
}
