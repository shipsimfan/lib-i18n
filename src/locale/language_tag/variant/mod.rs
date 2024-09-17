/// A variant subtag
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variant {
    tag: [u8; Self::MAX_LENGTH],
}

mod as_ref;
mod display;
mod eq;
mod iter;
mod new;

impl Variant {
    /// The maximum length a variant subtag can be
    pub const MAX_LENGTH: usize = 8;

    /// Gets the length of this variant
    pub const fn len(&self) -> usize {
        let mut len = 0;
        while len < Self::MAX_LENGTH && self.tag[len] != 0 {
            len += 1;
        }
        len
    }

    /// Gets the variant as a [`u8`] slice
    pub fn as_slice(&self) -> &[u8] {
        &self.tag[..self.len()]
    }

    /// Gets the variant as a [`str`]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_slice()) }
    }
}
