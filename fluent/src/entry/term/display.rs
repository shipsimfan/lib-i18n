use crate::FluentTerm;

impl core::fmt::Display for FluentTerm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "-{} = {}", self.name, self.pattern)?;

        for attribute in &self.attributes {
            attribute.fmt(f)?;
        }

        writeln!(f)
    }
}
