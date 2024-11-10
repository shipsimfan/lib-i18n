use crate::Variant;

impl Variant {
    /// Creates a valid set of variants from a slice
    #[cfg(not(feature = "alloc"))]
    pub const fn from_slice(slice: &[Variant]) -> &[Variant] {
        slice
    }

    /// Creates a valid set of variants from a slice
    #[cfg(feature = "alloc")]
    pub const fn from_slice(slice: &[Variant]) -> alloc::borrow::Cow<[Variant]> {
        alloc::borrow::Cow::Borrowed(slice)
    }
}
