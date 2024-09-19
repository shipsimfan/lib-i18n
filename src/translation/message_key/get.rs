use crate::{
    locale::LanguageTag,
    translation::{Message, MessageKey},
};
use std::borrow::Cow;

impl<'a, A> MessageKey<'a, A> {
    /// Gets a version of this message in the requested language or best fallback option
    pub fn get(&self, mut language: LanguageTag) -> &Message<A> {
        if let Some(message) = self.try_get(&language) {
            return message;
        }

        while apply_fallback(&mut language) {
            if let Some(message) = self.try_get(&language) {
                return message;
            }
        }

        &self.messages[0].1
    }

    /// Gets a version of this message in the requested language if it is available
    pub fn try_get(&self, language: &LanguageTag) -> Option<&Message<A>> {
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
    if language.variants.len() > 0 {
        language.variants = Cow::Owned(Vec::new());
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
