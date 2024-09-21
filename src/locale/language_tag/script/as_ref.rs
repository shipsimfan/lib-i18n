use crate::locale::Script;

impl AsRef<[u8]> for Script {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsRef<str> for Script {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl core::ops::Deref for Script {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
