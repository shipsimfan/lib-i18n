use crate::FluentPatternElement;

impl FluentPatternElement {
    /// Creates a new [`FluentPatternElement`]
    pub fn new<T: Into<FluentPatternElement>>(value: T) -> Self {
        value.into()
    }
}
