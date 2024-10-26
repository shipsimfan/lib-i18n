use crate::fluent::FluentInlinePlaceable;

impl core::fmt::Display for FluentInlinePlaceable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ {} }}", self.expression)
    }
}
