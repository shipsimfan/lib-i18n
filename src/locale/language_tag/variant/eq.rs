use crate::locale::Variant;

impl PartialEq<[u8]> for Variant {
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

impl PartialEq<Variant> for [u8] {
    fn eq(&self, other: &Variant) -> bool {
        other == self
    }
}

impl PartialEq<str> for Variant {
    fn eq(&self, other: &str) -> bool {
        self == other.as_bytes()
    }
}

impl PartialEq<Variant> for str {
    fn eq(&self, other: &Variant) -> bool {
        other == self
    }
}

impl PartialEq<String> for Variant {
    fn eq(&self, other: &String) -> bool {
        self == other.as_bytes()
    }
}

impl PartialEq<Variant> for String {
    fn eq(&self, other: &Variant) -> bool {
        other == self
    }
}
