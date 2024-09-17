mod language;
mod region;
mod script;
mod variant;

pub use language::Language;
pub use region::Region;
pub use script::Script;

/// A simplified BCP-47 language tag
pub struct LanguageTag {
    /// The primary language tag
    pub language: Language,

    /// The desired script for text
    pub script: Option<Script>,

    /// The region the speaker's dialect is from
    pub region: Option<Region>,
}
