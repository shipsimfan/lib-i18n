use crate::fluent::FluentPosition;

impl FluentPosition {
    /// Creates a new [`FluentPosition`] at the start of a file
    pub(in crate::fluent) const fn new(line: u32, column: u32) -> Self {
        assert!(line != 0);
        assert!(column != 0);

        FluentPosition { line, column }
    }
}
