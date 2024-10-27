use crate::FluentExpression;

impl core::fmt::Display for FluentExpression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FluentExpression::Inline(inline) => inline.fmt(f),
        }
    }
}
