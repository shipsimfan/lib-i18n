use crate::{FluentIdentifier, FluentVariableReference};
use core::ops::Deref;

impl Deref for FluentVariableReference {
    type Target = FluentIdentifier;

    fn deref(&self) -> &Self::Target {
        self.name()
    }
}

impl AsRef<FluentIdentifier> for FluentVariableReference {
    fn as_ref(&self) -> &FluentIdentifier {
        self.name()
    }
}
