use crate::{FluentPattern, FluentPatternElement};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

impl FluentPattern {
    /// Creates a new [`FluentPattern`]
    pub fn new<T: Into<FluentPatternElement>>(elements: Vec<T>) -> Self {
        assert!(elements.len() > 0);

        let elements = elements.into_iter().map(Into::into).collect();

        FluentPattern { elements }
    }
}
