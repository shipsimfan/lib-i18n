/// The primary language subtag
pub struct Language<'a> {
    tag: &'a [u8],
}

mod new;

impl<'a> Language<'a> {
    /// The maximum length a language subtag can be
    pub const MAX_LENGTH: usize = 8;

    /// Gets the language as a [`u8`] slice
    pub fn as_slice(&self) -> &'a [u8] {
        self.tag
    }

    /// Gets the language as a [`str`]
    pub fn as_str(&self) -> &'a str {
        unsafe { std::str::from_utf8_unchecked(self.tag) }
    }
}
