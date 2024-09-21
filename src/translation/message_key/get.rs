use crate::{
    locale::LanguageTag,
    translation::{Message, MessageKey},
};

impl<'a, A> MessageKey<'a, A> {
    /// Gets the message in the default locale
    pub fn default<'b>(&'b self) -> &'b Message<A> {
        &self.messages[0].1
    }

    /// Gets a version of this message in the requested language or best fallback option
    pub fn get<'b, 'c>(&'b self, mut language: LanguageTag<'c>) -> &'b Message<A> {
        if let Some(message) = self.try_get(&language) {
            return message;
        }

        while apply_fallback(&mut language) {
            if let Some(message) = self.try_get(&language) {
                return message;
            }
        }

        self.default()
    }

    /// Gets a version of this message in the requested language if it is available
    pub fn try_get<'b>(&'b self, language: &LanguageTag) -> Option<&'b Message<A>> {
        for (msg_language, message) in &self.messages {
            if language == *msg_language {
                return Some(message);
            }
        }

        None
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
