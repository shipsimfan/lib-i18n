use crate::fluent::FluentAttribute;

impl core::fmt::Display for FluentAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        write!(f, "    .{} = {}", self.name, self.pattern)
    }
}
