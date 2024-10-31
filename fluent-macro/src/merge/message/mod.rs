use fluent::{FluentPattern, FluentResource};
use locale::LanguageTag;
use std::collections::HashMap;

mod get;

/// A single merged message
pub struct MergedMessage<'a> {
    /// The messages for each language, with a reference to their original resource for rendering
    languages: HashMap<LanguageTag<'static>, (&'a FluentPattern, &'a FluentResource)>,
}
