use crate::locale::Region;

impl PartialEq<[u8]> for Region {
    fn eq(&self, other: &[u8]) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for (a, b) in core::iter::zip(self, other) {
            if *a != *b {
                return false;
            }
        }

        true
    }
}

impl PartialEq<Region> for [u8] {
    fn eq(&self, other: &Region) -> bool {
        other == self
    }
}

impl PartialEq<str> for Region {
    fn eq(&self, other: &str) -> bool {
        self == other.as_bytes()
    }
}

impl PartialEq<Region> for str {
    fn eq(&self, other: &Region) -> bool {
        other == self
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<alloc::string::String> for Region {
    fn eq(&self, other: &alloc::string::String) -> bool {
        self == other.as_bytes()
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<Region> for alloc::string::String {
    fn eq(&self, other: &Region) -> bool {
        other == self
    }
}
