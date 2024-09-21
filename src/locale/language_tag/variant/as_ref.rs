use crate::locale::Variant;

impl AsRef<[u8]> for Variant {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsRef<str> for Variant {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl core::ops::Deref for Variant {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
