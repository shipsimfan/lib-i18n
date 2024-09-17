/// A region or country subtag
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Region {
    tag: [u8; Self::MAX_LENGTH],
}

mod as_ref;
mod display;
mod eq;
mod iter;
mod new;

impl Region {
    /// The maximum length a region subtag can be
    pub const MAX_LENGTH: usize = 3;

    /// The minimum length a region subtag can be
    pub const MIN_LENGTH: usize = 2;

    /// Gets the length of this region
    pub const fn len(&self) -> usize {
        let mut len = 0;
        while len < Self::MAX_LENGTH && self.tag[len] != 0 {
            len += 1;
        }
        len
    }

    /// Gets the region as a [`u8`] slice
    pub fn as_slice(&self) -> &[u8] {
        &self.tag[..self.len()]
    }

    /// Gets the region as a [`str`]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_slice()) }
    }
}
