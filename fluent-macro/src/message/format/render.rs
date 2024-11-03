use crate::IncludeFluentFormat;
use i18n_fluent::{FluentPattern, FluentPatternElement, FluentResource};
use proc_macro_util::{
    tokens::{Identifier, Literal},
    Error, Result,
};

impl IncludeFluentFormat {
    /// Renders `pattern` into an [`IncludeFluentFormat`], using references in `resource` for
    /// placeables
    pub fn render(
        language: Identifier,
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

        Ok(IncludeFluentFormat {
            language,
            string: Literal::new(string.as_str()),
        })
    }
}
