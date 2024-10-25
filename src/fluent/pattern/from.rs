use crate::fluent::{FluentPattern, FluentPatternElement};

impl<T: Into<FluentPatternElement>> From<T> for FluentPattern {
    fn from(value: T) -> Self {
        FluentPattern::new(vec![value.into()])
    }
}

impl<T: Into<FluentPatternElement>> From<Vec<T>> for FluentPattern {
    fn from(value: Vec<T>) -> Self {
        FluentPattern::new(value)
    }
}
