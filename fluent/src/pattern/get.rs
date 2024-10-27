use crate::{FluentPattern, FluentPatternElement, FluentPosition};

impl FluentPattern {
    /// Gets the position of the start of this pattern
    pub fn position(&self) -> FluentPosition {
        self.elements[0].position()
    }

    /// Gets the elements that make up this pattern
    pub fn elements(&self) -> &[FluentPatternElement] {
        &self.elements
    }
}
