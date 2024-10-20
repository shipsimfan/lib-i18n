use crate::fluent::FluentPosition;

impl FluentPosition {
    /// Creates a new [`FluentPosition`] at the start of a file
    pub(in crate::fluent) const fn new() -> Self {
        FluentPosition { line: 1, column: 1 }
    }
}
