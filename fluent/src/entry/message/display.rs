use crate::FluentMessage;

impl core::fmt::Display for FluentMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} = ", self.name)?;

        if let Some(pattern) = self.pattern.as_ref() {
            pattern.fmt(f)?;
        }

        for attribute in &self.attributes {
            attribute.fmt(f)?;
        }

        writeln!(f)
    }
}
