use crate::locale::Region;

impl Region {
    /// Creates an iterator over the bytes of this region
    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.as_slice().iter()
    }
}

impl<'a> IntoIterator for &'a Region {
    type IntoIter = std::slice::Iter<'a, u8>;
    type Item = &'a u8;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
