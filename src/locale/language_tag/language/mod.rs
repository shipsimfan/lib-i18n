/// A primary language subtag
pub struct Language {
    tag: [u8; Self::MAX_LENGTH],
}

mod new;

impl Language {
    /// The maximum length a language subtag can be
    pub const MAX_LENGTH: usize = 8;

    /// Gets the length of this language
    pub const fn len(&self) -> usize {
        let mut len = 0;
        while len < Self::MAX_LENGTH && self.tag[len] != 0 {
            len += 1;
        }
        len
    }

    /// Gets the language as a [`u8`] slice
    pub fn as_slice(&self) -> &[u8] {
        &self.tag
    }

    /// Gets the language as a [`str`]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_slice()) }
    }
}
