/// A subtag denoting the desired script
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Script {
    tag: [u8; Self::LENGTH],
}

mod as_ref;
mod display;
mod eq;
mod iter;
mod new;

impl Script {
    /// The length a script subtag must be
    pub const LENGTH: usize = 4;

    /// Gets the length of this script
    pub const fn len(&self) -> usize {
        Self::LENGTH
    }

    /// Gets the script as a [`u8`] slice
    pub fn as_slice(&self) -> &[u8] {
        &self.tag
    }

    /// Gets the script as a [`str`]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_slice()) }
    }
}
