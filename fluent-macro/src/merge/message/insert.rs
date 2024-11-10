use crate::{IncludeFluentVariable, MergedMessage};
use i18n_fluent::{
    FluentExpression, FluentIdentifier, FluentInlineExpression, FluentPattern,
    FluentPatternElement, FluentResource,
};
use i18n_locale::LanguageTag;
use proc_macro_util::{Error, Result};

impl<'a> MergedMessage<'a> {
    /// Inserts a new `pattern` for a given `langauge`, returning true if another pattern was already in place for that language
    pub fn insert(
        &mut self,
        name: &FluentIdentifier,
        language: &'a LanguageTag<'static>,
        pattern: &'a FluentPattern,
        resource: &'a FluentResource,
    ) -> Result<()> {
        for element in pattern {
            let inline_placeable = match element {
                FluentPatternElement::InlinePlaceable(inline_placeable) => inline_placeable,
                FluentPatternElement::BlockPlaceable(block_placeable) => block_placeable,
                _ => continue,
            };

            match inline_placeable.expression() {
                FluentExpression::Inline(FluentInlineExpression::VariableReference(
                    variable_reference,
                )) => {
                    match self.variables.insert(
                        variable_reference.name().to_string(),
                        IncludeFluentVariable::Display,
                    ) {
                        Some(IncludeFluentVariable::Display) | None => {}
                        _ => {
                            return Err(Error::new(format_args!(
                                "conflicting types for variable \"{}\" in message \"{}\"",
                                variable_reference, name
                            )))
                        }
                    }
                }
                _ => {
                    return Err(Error::new(
                        "only variable reference placeables are currently support",
                    ))
                }
            }
        }

        match self.languages.insert(language, (pattern, resource)) {
            Some(_) => Err(Error::new(format_args!(
                "more than one message defined for key \"{}\" in language \"{}\"",
                name, language,
            ))),
            None => Ok(()),
        }
    }
}
