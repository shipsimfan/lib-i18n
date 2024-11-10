/// A variant subtag
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variant {
    tag: [u8; Self::MAX_LENGTH],
}

mod as_ref;
mod display;
mod eq;
mod from_slice;
mod iter;
mod new;

#[cfg(test)]
mod tests;

impl Variant {
    /// The maximum length a variant subtag can be
    pub const MAX_LENGTH: usize = 8;

    /// Gets the length of this variant
    pub const fn len(&self) -> usize {
        (u64::from_le_bytes(self.tag).ilog2() as usize / 8) + 1
    }

    /// Gets the variant as a [`u8`] slice
    pub fn as_slice(&self) -> &[u8] {
        &self.tag[..self.len()]
    }

    /// Gets the variant as a [`str`]
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.as_slice()) }
    }
}
