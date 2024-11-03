use crate::SupportedLanguage;
use std::hash::Hash;

impl Hash for SupportedLanguage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}
