use crate::locale::LanguageTag;

impl std::fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
