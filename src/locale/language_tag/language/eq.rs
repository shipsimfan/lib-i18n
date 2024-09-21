use crate::locale::Language;

impl PartialEq<[u8]> for Language {
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

impl PartialEq<Language> for [u8] {
    fn eq(&self, other: &Language) -> bool {
        other == self
    }
}

impl PartialEq<str> for Language {
    fn eq(&self, other: &str) -> bool {
        self == other.as_bytes()
    }
}

impl PartialEq<Language> for str {
    fn eq(&self, other: &Language) -> bool {
        other == self
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<alloc::string::String> for Language {
    fn eq(&self, other: &alloc::string::String) -> bool {
        self == other.as_bytes()
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<Language> for alloc::string::String {
    fn eq(&self, other: &Language) -> bool {
        other == self
    }
}
