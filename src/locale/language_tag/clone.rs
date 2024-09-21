use crate::locale::LanguageTag;

impl<'a> LanguageTag<'a> {
    /// Clones this [`LanguageTag`] without deep-copying the variants
    pub fn clone(&self) -> LanguageTag {
        LanguageTag {
            language: self.language,
            script: self.script,
            region: self.region,
            variants: self.variants.as_ref().into(),
        }
    }

    /// Converts this [`LanguageTag`] into a owned version
    #[cfg(feature = "alloc")]
    pub fn to_owned(&self) -> LanguageTag<'static> {
        LanguageTag {
            language: self.language,
            script: self.script,
            region: self.region,
            variants: self.variants.as_ref().to_vec().into(),
        }
    }
}

impl LanguageTag<'static> {
    /// Clones a [`LanguageTag`] with a `'static` lifetime
    pub fn static_clone(&self) -> LanguageTag<'static> {
        LanguageTag {
            language: self.language,
            script: self.script,
            region: self.region,
            #[cfg(feature = "alloc")]
            variants: self.variants.clone(),
            #[cfg(not(feature = "alloc"))]
            variants: self.variants,
        }
    }
}
