use crate::fluent::{FluentPattern, FluentPatternElement};

impl FluentPattern {
    /// Creates a new [`FluentPattern`]
    pub fn new<T: Into<FluentPatternElement>>(elements: Vec<T>) -> Self {
        assert!(elements.len() > 0);

        let elements = elements.into_iter().map(Into::into).collect();

        FluentPattern { elements }
    }
}
