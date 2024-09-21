/// An invalid language tag was parsed
#[derive(Debug, PartialEq, Eq)]
pub enum InvalidLanguageTag {
    /// The language identifier was not valid
    InvalidLanguage,

    /// The script was not valid
    InvalidScript,

    /// The region was not valid
    InvalidRegion,

    /// The variant was not valid
    InvalidVariant,
}

impl core::error::Error for InvalidLanguageTag {}

impl core::fmt::Display for InvalidLanguageTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            InvalidLanguageTag::InvalidLanguage => "invalid language",
            InvalidLanguageTag::InvalidScript => "invalid script",
            InvalidLanguageTag::InvalidRegion => "invalid region",
            InvalidLanguageTag::InvalidVariant => "invalid variant",
        })
    }
}
