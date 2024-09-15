mod extended_language;
mod extension;
mod language;
mod private_use;
mod region;
mod script;
mod variant;

pub use language::Language;

/// A BCP-47 language tag
pub struct LanguageTag<'a> {
    /// The primary language tag
    pub language: Language<'a>,
}
