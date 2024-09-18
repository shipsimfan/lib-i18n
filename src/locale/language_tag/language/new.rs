use crate::locale::Language;

impl Language {
    /// Creates new [`Language`] from `tag` without validating its contents
    pub const unsafe fn new_unchecked(tag: &[u8]) -> Self {
        assert!(tag.len() >= Self::MIN_LENGTH);
        assert!(tag.len() <= Self::MAX_LENGTH);

        let mut owned_tag = [0; Self::MAX_LENGTH];
        let mut i = 0;
        while i < tag.len() {
            owned_tag[i] = tag[i].to_ascii_lowercase();
            i += 1;
        }

        Language { tag: owned_tag }
    }

    /// Creates new [`Language`] from `tag` validating its contents
    pub const fn new(tag: &[u8]) -> Option<Self> {
        if tag.len() < Self::MIN_LENGTH || tag.len() > Self::MAX_LENGTH {
            return None;
        }

        let mut owned_tag = [0; Self::MAX_LENGTH];
        let mut i = 0;
        while i < tag.len() {
            if !tag[i].is_ascii_alphanumeric() {
                return None;
            }

            owned_tag[i] = tag[i].to_ascii_lowercase();
            i += 1;
        }

        Some(Language { tag: owned_tag })
    }
}
