use crate::fluent::FluentPosition;

impl core::fmt::Display for FluentPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
