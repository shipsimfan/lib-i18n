use crate::locale::Region;

impl Region {
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
