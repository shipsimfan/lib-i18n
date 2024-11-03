use crate::SupportedLanguage;

impl PartialEq for SupportedLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.tag.eq(&other.tag)
    }
}

impl Eq for SupportedLanguage {}
