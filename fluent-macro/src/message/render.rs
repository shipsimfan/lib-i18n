use crate::{IncludeFluentMessage, MergedMessage};
use fluent::FluentIdentifier;
use locale::LanguageTag;
use proc_macro_util::Result;
use std::collections::HashSet;

impl IncludeFluentMessage {
    /// Renders a module into message keys
    pub fn render(
        message: &MergedMessage,
        name: &FluentIdentifier,
        supported_languages: &mut HashSet<LanguageTag<'static>>,
    ) -> Result<Self> {
        todo!()
    }
}
