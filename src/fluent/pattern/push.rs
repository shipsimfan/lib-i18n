use crate::fluent::{FluentPattern, FluentPatternElement};

impl FluentPattern {
    /// Pushes `element` to the end of the [`FluentPattern`]
    pub fn push<T: Into<FluentPatternElement>>(&mut self, element: T) {
        let element = element.into();

        if let Some(last) = self.last() {
            assert!(last.position() < element.position());
        }

        self.elements.push(element);
    }
}
