use crate::fluent::{FluentPattern, FluentPatternElement};

impl FluentPattern {
    /// Creates a new [`FluentPattern`]
    pub fn new(elements: Vec<FluentPatternElement>) -> Self {
        assert!(elements.len() > 0);

        FluentPattern { elements }
    }
}
