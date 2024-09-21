use crate::locale::LanguageTag;

impl<'a> core::fmt::Display for LanguageTag<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.language.fmt(f)?;

        if let Some(script) = self.script {
            write!(f, "-{}", script)?;
        }

        if let Some(region) = self.region {
            write!(f, "-{}", region)?;
        }

        for variant in self.variants.iter() {
            write!(f, "-{}", variant)?;
        }

        Ok(())
    }
}
