use crate::{Message, MessageDisplay};
use i18n_locale::LanguageTag;

mod r#macro;

/// A set of [`Message`]s with the same meaning in multiple languages
pub trait MessageKey {
    /// The arguments passed to messages of this key
    type Arguments<'a>: Sized;

    /// Attempts to get a message display in the given `language` exactly
    fn try_get_message<'a>(language: &LanguageTag) -> Option<Message<Self::Arguments<'a>>>;

    /// Gets the message display in the default locale
    fn default_message<'a>() -> Message<Self::Arguments<'a>>;

    /// Gets the message in the default locale
    fn default<'a>(arguments: Self::Arguments<'a>) -> MessageDisplay<Self::Arguments<'a>> {
        MessageDisplay::new(Self::default_message(), arguments)
    }

    /// Gets the best message display for the given `language`
    fn get<'a>(
        mut language: LanguageTag,
        arguments: Self::Arguments<'a>,
    ) -> MessageDisplay<Self::Arguments<'a>> {
        if let Some(message) = Self::try_get_message(&language) {
            return MessageDisplay::new(message, arguments);
        }

        while apply_fallback(&mut language) {
            if let Some(message) = Self::try_get_message(&language) {
                return MessageDisplay::new(message, arguments);
            }
        }

        Self::default(arguments)
    }

    /// Attempts to get a message in the given `language` exactly
    fn try_get<'a>(
        language: &LanguageTag,
        arguments: Self::Arguments<'a>,
    ) -> Option<MessageDisplay<Self::Arguments<'a>>> {
        Self::try_get_message(language).map(|message| MessageDisplay::new(message, arguments))
    }
}

/// Attempts to fallback the given language, returning if it was able to
fn apply_fallback(language: &mut LanguageTag) -> bool {
    #[cfg(feature = "alloc")]
    if language.variants.len() > 0 {
        language.variants = alloc::borrow::Cow::Owned(alloc::vec::Vec::new());
        return true;
    }

    if language.region.is_some() {
        language.region = None;
        return true;
    }

    if language.script.is_some() {
        language.script = None;
        return true;
    }

    false
}
