use crate::FluentBlockPlaceable;

impl core::fmt::Display for FluentBlockPlaceable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        write!(f, "    {}", self.placeable)
    }
}
