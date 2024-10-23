use crate::fluent::FluentPattern;

impl core::fmt::Display for FluentPattern {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for element in &self.elements {
            element.fmt(f)?;
        }

        Ok(())
    }
}
