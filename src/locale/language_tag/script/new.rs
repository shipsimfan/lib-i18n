use crate::locale::Script;

impl Script {
    /// Creates new [`Script`] from `tag` without validating its contents
    pub const unsafe fn new_unchecked(tag: &[u8]) -> Self {
        assert!(tag.len() == Self::LENGTH);

        let mut owned_tag = [0; Self::LENGTH];
        let mut i = 0;
        while i < tag.len() {
            owned_tag[i] = if i == 0 {
                tag[i].to_ascii_uppercase()
            } else {
                tag[i].to_ascii_lowercase()
            };
            i += 1;
        }

        Script { tag: owned_tag }
    }

    /// Creates new [`Script`] from `tag` validating its contents
    pub const fn new(tag: &[u8]) -> Option<Self> {
        if tag.len() != Self::LENGTH {
            return None;
        }

        let mut owned_tag = [0; Self::LENGTH];
        let mut i = 0;
        while i < tag.len() {
            if !tag[i].is_ascii_alphanumeric() {
                return None;
            }

            owned_tag[i] = if i == 0 {
                tag[i].to_ascii_uppercase()
            } else {
                tag[i].to_ascii_lowercase()
            };
            i += 1;
        }

        Some(Script { tag: owned_tag })
    }
}
