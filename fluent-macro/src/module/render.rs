use crate::{IncludeFluentMessage, IncludeFluentModule, MergedModule, SupportedLanguage};
use proc_macro_util::{tokens::Identifier, Result};
use std::collections::HashSet;

impl IncludeFluentModule {
    /// Renders a module into message keys
    pub fn render(
        module: &MergedModule,
        depth: usize,
        supported_languages: &mut HashSet<SupportedLanguage>,
    ) -> Result<Self> {
        let sub_modules = module
            .sub_modules()
            .into_iter()
            .map(|(name, sub_module)| {
                Ok((
                    Identifier::new(&name.to_string()),
                    IncludeFluentModule::render(sub_module, depth + 1, supported_languages)?,
                ))
            })
            .collect::<Result<_>>()?;

        let messages = module
            .messages()
            .into_iter()
            .map(|(name, message)| IncludeFluentMessage::render(message, name, supported_languages))
            .collect::<Result<_>>()?;

        Ok(IncludeFluentModule {
            depth,
            sub_modules,
            messages,
        })
    }
}
