use locale::LanguageTag;

/// A format string used to display a message in a language
pub struct IncludeFluentFormat {
    /// The language to display
    language: LanguageTag<'static>,

    /// The actual format string
    string: String,
    // TODO: Add inserts
}
