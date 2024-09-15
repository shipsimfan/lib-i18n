use crate::locale::Language;

impl<'a> Language<'a> {
    /// Creates a new [`Language`] without checking the contents
    pub const unsafe fn new_unchecked(tag: &'a [u8]) -> Self {
        assert!(tag.len() <= Self::MAX_LENGTH);
        assert!(tag.len() > 0);
        Language { tag }
    }

    /// Creates new [`Language`], checking if the contents are valid
    pub fn new<T: AsRef<[u8]>>(tag: &'a T) -> Option<Self> {
        let tag = tag.as_ref();
        if tag.len() == 0 || tag.len() > Self::MAX_LENGTH {
            return None;
        }

        for byte in tag {
            if !byte.is_ascii_alphanumeric() {
                return None;
            }
        }

        Some(unsafe { Language::new_unchecked(tag) })
    }
}
