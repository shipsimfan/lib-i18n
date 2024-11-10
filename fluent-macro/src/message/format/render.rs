use super::insert::IncludeFluentFormatInsert;
use crate::IncludeFluentFormat;
use i18n_fluent::{
    FluentExpression, FluentInlineExpression, FluentPattern, FluentPatternElement, FluentResource,
};
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
        let mut inserts = Vec::new();

        for element in pattern {
            match element {
                FluentPatternElement::InlineText(inline_text) => string.push_str(&inline_text),
                FluentPatternElement::InlinePlaceable(inline_placeable) => {
                    match inline_placeable.expression() {
                        FluentExpression::Inline(FluentInlineExpression::VariableReference(
                            variable,
                        )) => {
                            string.push_str("{}");
                            inserts.push(IncludeFluentFormatInsert::new(variable.name().as_ref()));
                        }
                        _ => {
                            return Err(Error::new(
                                "only variable references are currently supported",
                            ))
                        }
                    }
                }
                _ => return Err(Error::new("only inline patterns are currently supported")),
            }
        }

        Ok(IncludeFluentFormat {
            language,
            string: Literal::new(string.as_str()),
            inserts,
        })
    }
}
