use crate::locale::Region;

impl Region {
    /// Creates new [`Region`] from `tag` without validating its contents
    pub const unsafe fn new_unchecked(tag: &[u8]) -> Self {
        assert!(tag.len() >= Self::MIN_LENGTH);
        assert!(tag.len() <= Self::MAX_LENGTH);

        let mut owned_tag = [0; Self::LENGTH];
        let mut i = 0;
        while i < tag.len() {
            owned_tag[i] = tag[i].to_ascii_uppercase();
            i += 1;
        }

        Region { tag: owned_tag }
    }

    /// Creates new [`Region`] from `tag` validating its contents
    pub const fn new(tag: &[u8]) -> Option<Self> {
        if tag.len() < Self::MIN_LENGTH || tag.len() > Self::MAX_LENGTH {
            return None;
        }

        let mut owned_tag = [0; Self::LENGTH];
        let mut i = 0;
        while i < tag.len() {
            if !tag[i].is_ascii_alphanumeric() {
                return None;
            }

            owned_tag[i] = tag[i].to_ascii_uppercase();
            i += 1;
        }

        Some(Region { tag: owned_tag })
    }
}
