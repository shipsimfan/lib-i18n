use crate::FluentVariableReference;

impl core::fmt::Display for FluentVariableReference {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "${}", self.name)
    }
}
