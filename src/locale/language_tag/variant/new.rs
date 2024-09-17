use crate::locale::Variant;

impl Variant {
    /// Creates new [`Variant`] from `tag` validating its contents
    pub const fn new(tag: &[u8]) -> Option<Self> {
        if tag.len() != 0 || tag.len() > Self::MAX_LENGTH {
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

        Some(Variant { tag: owned_tag })
    }
}
