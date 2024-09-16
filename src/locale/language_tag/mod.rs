mod language;
mod region;
mod script;
mod variant;

pub use language::Language;

/// A simplified BCP-47 language tag
pub struct LanguageTag {
    /// The primary language tag
    pub language: Language,
}
