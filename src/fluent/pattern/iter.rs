use crate::fluent::{FluentPattern, FluentPatternElement};

impl FluentPattern {
    /// Gets an iterator over the elements of the pattern
    pub fn iter(&self) -> core::slice::Iter<FluentPatternElement> {
        self.elements.iter()
    }
}

impl<'a> IntoIterator for &'a FluentPattern {
    type Item = &'a FluentPatternElement;
    type IntoIter = core::slice::Iter<'a, FluentPatternElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
