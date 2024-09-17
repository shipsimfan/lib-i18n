/// A subtag denoting the desired script
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Script {
    tag: [u8; Self::MAX_LENGTH],
}

mod as_ref;
mod display;
mod eq;
mod iter;
mod new;

impl Script {
    /// The maximum length a script subtag can be
    pub const MAX_LENGTH: usize = 4;

    /// Gets the length of this script
    pub const fn len(&self) -> usize {
        let mut len = 0;
        while len < Self::MAX_LENGTH && self.tag[len] != 0 {
            len += 1;
        }
        len
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
