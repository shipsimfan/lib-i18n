mod language;
mod region;
mod script;
mod variant;

mod clone;
mod display;
mod new;

pub use language::Language;
pub use new::InvalidLanguageTag;
pub use region::Region;
pub use script::Script;
pub use variant::Variant;

/// A simplified BCP-47 language tag
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageTag<'a> {
    /// The primary language tag
    pub language: Language,

    /// The desired script for text
    pub script: Option<Script>,

    /// The region the speaker's dialect is from
    pub region: Option<Region>,

    /// The variants of the specific language
    #[cfg(feature = "alloc")]
    pub variants: alloc::borrow::Cow<'a, [Variant]>,

    /// The variants of the specific language
    #[cfg(not(feature = "alloc"))]
    pub variants: &'a [Variant],
}
