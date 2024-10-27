use crate::FluentBlockText;

impl core::fmt::Display for FluentBlockText {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        write!(f, "    {}", self.content)
    }
}
