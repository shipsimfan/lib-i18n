use crate::IncludeFluentFormat;
use fluent::{FluentPattern, FluentPatternElement, FluentResource};
use locale::LanguageTag;
use proc_macro_util::{Error, Result};

impl IncludeFluentFormat {
    /// Renders `pattern` into an [`IncludeFluentFormat`], using references in `resource` for
    /// placeables
    pub fn render(
        language: LanguageTag<'static>,
        pattern: &FluentPattern,
        resource: &FluentResource,
    ) -> Result<Self> {
        let mut string = String::new();

        for element in pattern {
            match element {
                FluentPatternElement::InlineText(inline_text) => string.push_str(&inline_text),
                _ => {
                    return Err(Error::new(
                        "only inline text patterns are currently supported",
                    ))
                }
            }
        }

        Ok(IncludeFluentFormat { language, string })
    }
}
