use crate::locale::Language;

impl PartialEq<[u8]> for Language {
    fn eq(&self, other: &[u8]) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for (a, b) in std::iter::zip(self, other) {
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

impl PartialEq<String> for Language {
    fn eq(&self, other: &String) -> bool {
        self == other.as_bytes()
    }
}

impl PartialEq<Language> for String {
    fn eq(&self, other: &Language) -> bool {
        other == self
    }
}
