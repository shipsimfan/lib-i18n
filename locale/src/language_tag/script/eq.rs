use crate::Script;

impl PartialEq<[u8]> for Script {
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

impl PartialEq<Script> for [u8] {
    fn eq(&self, other: &Script) -> bool {
        other == self
    }
}

impl PartialEq<str> for Script {
    fn eq(&self, other: &str) -> bool {
        self == other.as_bytes()
    }
}

impl PartialEq<Script> for str {
    fn eq(&self, other: &Script) -> bool {
        other == self
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<alloc::string::String> for Script {
    fn eq(&self, other: &alloc::string::String) -> bool {
        self == other.as_bytes()
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<Script> for alloc::string::String {
    fn eq(&self, other: &Script) -> bool {
        other == self
    }
}
