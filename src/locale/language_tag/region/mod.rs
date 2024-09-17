/// A region or country subtag
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Region {
    tag: [u8; Self::LENGTH],
}

mod as_ref;
mod display;
mod eq;
mod iter;
mod new;

#[cfg(test)]
mod tests;

impl Region {
    /// The length of the underlying array
    const LENGTH: usize = 4;

    /// The maximum length a region subtag can be
    pub const MAX_LENGTH: usize = 3;

    /// The minimum length a region subtag can be
    pub const MIN_LENGTH: usize = 2;

    /// Gets the length of this region
    pub const fn len(&self) -> usize {
        (u32::from_le_bytes(self.tag).ilog2() as usize / 8) + 1
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
