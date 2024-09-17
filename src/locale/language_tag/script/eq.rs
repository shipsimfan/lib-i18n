use crate::locale::Script;

impl PartialEq<[u8]> for Script {
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

impl PartialEq<String> for Script {
    fn eq(&self, other: &String) -> bool {
        self == other.as_bytes()
    }
}

impl PartialEq<Script> for String {
    fn eq(&self, other: &Script) -> bool {
        other == self
    }
}
