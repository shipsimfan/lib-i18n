use crate::FluentPosition;

impl From<(u32, u32)> for FluentPosition {
    fn from(value: (u32, u32)) -> Self {
        FluentPosition::new(value.0, value.1)
    }
}
