use crate::locale::Variant;

impl Variant {
    /// Creates an iterator over the bytes of this variant
    pub fn iter(&self) -> core::slice::Iter<u8> {
        self.as_slice().iter()
    }
}

impl<'a> IntoIterator for &'a Variant {
    type IntoIter = core::slice::Iter<'a, u8>;
    type Item = &'a u8;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
