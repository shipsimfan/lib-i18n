use i18n_fluent::{FluentPattern, FluentResource};
use i18n_locale::LanguageTag;
use std::collections::HashMap;

mod get;
mod insert;
mod new;

/// A single merged message
pub struct MergedMessage<'a> {
    /// The messages for each language, with a reference to their original resource for rendering
    languages: HashMap<&'a LanguageTag<'static>, (&'a FluentPattern, &'a FluentResource)>,
}
