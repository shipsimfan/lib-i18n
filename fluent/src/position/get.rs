use crate::FluentPosition;

impl FluentPosition {
    /// The line the element is on
    pub fn line(&self) -> u32 {
        self.line
    }

    /// The column the element is at
    pub fn column(&self) -> u32 {
        self.column
    }
}
