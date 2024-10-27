use crate::{FluentPattern, FluentPatternElement};
use core::ops::Deref;

impl Deref for FluentPattern {
    type Target = [FluentPatternElement];

    fn deref(&self) -> &Self::Target {
        self.elements()
    }
}

impl AsRef<[FluentPatternElement]> for FluentPattern {
    fn as_ref(&self) -> &[FluentPatternElement] {
        self.elements()
    }
}
