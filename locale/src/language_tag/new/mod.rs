use crate::{Language, LanguageTag};

mod error;

#[cfg(test)]
mod tests;

pub use error::InvalidLanguageTag;

impl<'a> LanguageTag<'a> {
    /// Creates a new [`LanguageTag`] from a `tag` containing just a language
    pub const fn from_language(tag: &[u8]) -> Option<Self> {
        let language = match Language::new(tag) {
            Some(language) => language,
            None => return None,
        };

        Some(LanguageTag {
            language,
            script: None,
            region: None,
            #[cfg(feature = "alloc")]
            variants: alloc::borrow::Cow::Borrowed(&[]),
            #[cfg(not(feature = "alloc"))]
            variants: &[],
        })
    }

    /// Creates a new [`LanguageTag`] from a `tag` containing just a language without checking its contents
    pub const unsafe fn from_language_unchecked(tag: &[u8]) -> Self {
        LanguageTag {
            language: Language::new_unchecked(tag),
            script: None,
            region: None,
            #[cfg(feature = "alloc")]
            variants: alloc::borrow::Cow::Borrowed(&[]),
            #[cfg(not(feature = "alloc"))]
            variants: &[],
        }
    }

    /// Attempts to parse `tag` in to a new [`LanguageTag`]
    #[cfg(feature = "alloc")]
    pub fn new(tag: &[u8]) -> Result<Self, InvalidLanguageTag> {
        use crate::{Region, Script, Variant};

        let mut i = 0;

        let language =
            Language::new(next_chunk(tag, &mut i).ok_or(InvalidLanguageTag::InvalidLanguage)?)
                .ok_or(InvalidLanguageTag::InvalidLanguage)?;

        let mut chunk = next_chunk(tag, &mut i);
        let script = if let Some(cur_chunk) = chunk {
            if cur_chunk.len() == Script::LENGTH {
                let script = Script::new(cur_chunk).ok_or(InvalidLanguageTag::InvalidScript)?;
                chunk = next_chunk(tag, &mut i);
                Some(script)
            } else {
                None
            }
        } else {
            None
        };

        let region = if let Some(cur_chunk) = chunk {
            if cur_chunk.len() >= Region::MIN_LENGTH && cur_chunk.len() <= Region::MAX_LENGTH {
                let region = Region::new(cur_chunk).ok_or(InvalidLanguageTag::InvalidRegion)?;
                chunk = next_chunk(tag, &mut i);
                Some(region)
            } else {
                None
            }
        } else {
            None
        };

        let mut variants = alloc::vec::Vec::new();
        while let Some(variant) = chunk {
            variants.push(Variant::new(variant).ok_or(InvalidLanguageTag::InvalidVariant)?);
            chunk = next_chunk(tag, &mut i);
        }
        let variants = alloc::borrow::Cow::Owned(variants);

        Ok(LanguageTag {
            language,
            script,
            region,
            variants,
        })
    }
}

#[cfg(feature = "alloc")]
fn next_chunk<'a>(tag: &'a [u8], i: &mut usize) -> Option<&'a [u8]> {
    if *i >= tag.len() {
        return None;
    }

    if *i > 0 {
        assert!(tag[*i] == b'-' || tag[*i] == b'_');
        *i += 1;
    }

    let start = *i;
    while *i < tag.len() && tag[*i] != b'-' && tag[*i] != b'_' {
        *i += 1;
    }

    Some(&tag[start..*i])
}
