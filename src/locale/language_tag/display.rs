use crate::locale::LanguageTag;

impl std::fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.language.fmt(f)?;

        if let Some(script) = self.script {
            write!(f, "-{}", script)?;
        }

        if let Some(region) = self.script {
            write!(f, "-{}", region)?;
        }

        if let Some(variants) = &self.variants {
            for variant in variants {
                write!(f, "-{}", variant)?;
            }
        }

        Ok(())
    }
}
