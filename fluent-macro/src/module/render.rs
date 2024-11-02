use crate::{message::IncludeFluentMessage, IncludeFluentModule, MergedModule};
use locale::LanguageTag;
use proc_macro_util::Result;
use std::collections::HashSet;

impl IncludeFluentModule {
    /// Renders a module into message keys
    pub fn render(
        module: &MergedModule,
        supported_languages: &mut HashSet<LanguageTag<'static>>,
    ) -> Result<Self> {
        let sub_modules = module
            .sub_modules()
            .into_iter()
            .map(|(name, sub_module)| {
                Ok((
                    name.to_string(),
                    IncludeFluentModule::render(sub_module, supported_languages)?,
                ))
            })
            .collect::<Result<_>>()?;

        let messages = module
            .messages()
            .into_iter()
            .map(|(name, message)| IncludeFluentMessage::render(message, name, supported_languages))
            .collect::<Result<_>>()?;

        Ok(IncludeFluentModule {
            sub_modules,
            messages,
        })
    }
}
