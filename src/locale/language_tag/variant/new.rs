use crate::locale::Variant;

impl Variant {
    /// Creates new [`Variant`] from `tag` without validating its contents
    pub const unsafe fn new_unchecked(tag: &[u8]) -> Self {
        assert!(tag.len() > 0);
        assert!(tag.len() <= Self::MAX_LENGTH);

        let mut owned_tag = [0; Self::MAX_LENGTH];
        let mut i = 0;
        while i < tag.len() {
            owned_tag[i] = tag[i];
            i += 1;
        }

        Variant { tag: owned_tag }
    }

    /// Creates new [`Variant`] from `tag` validating its contents
    pub const fn new(tag: &[u8]) -> Option<Self> {
        if tag.len() == 0 || tag.len() > Self::MAX_LENGTH {
            return None;
        }

        let mut owned_tag = [0; Self::MAX_LENGTH];
        let mut i = 0;
        while i < tag.len() {
            if !tag[i].is_ascii_alphanumeric() {
                return None;
            }

            owned_tag[i] = tag[i];
            i += 1;
        }

        Some(Variant { tag: owned_tag })
    }
}
