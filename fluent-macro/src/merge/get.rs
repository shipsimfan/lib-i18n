use crate::{MergedMessage, MergedModule};
use fluent::FluentIdentifier;
use std::collections::HashMap;

impl<'a> MergedModule<'a> {
    /// Gets the submodules that make up this module
    pub fn sub_modules(&self) -> &[(&str, MergedModule<'a>)] {
        &self.sub_modules
    }

    /// Gets the set of messages for this module
    pub fn messages(&self) -> &HashMap<&'a FluentIdentifier, MergedMessage<'a>> {
        &self.messages
    }
}
