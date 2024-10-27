use crate::{FluentBlockPlaceable, FluentInlinePlaceable};
use core::ops::Deref;

impl Deref for FluentBlockPlaceable {
    type Target = FluentInlinePlaceable;

    fn deref(&self) -> &Self::Target {
        self.placeable()
    }
}

impl AsRef<FluentInlinePlaceable> for FluentBlockPlaceable {
    fn as_ref(&self) -> &FluentInlinePlaceable {
        self.placeable()
    }
}
