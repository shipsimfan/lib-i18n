use crate::{LoadedDirectory, LoadedEntry, MergedMessage, MergedModule};
use i18n_fluent::{FluentEntry, FluentIdentifier, FluentResource};
use i18n_locale::LanguageTag;
use proc_macro_util::{Error, Result};
use std::collections::HashMap;

impl<'a> MergedModule<'a> {
    /// Merge all the messages with identical keys
    pub fn merge_entry(entry: &'a LoadedEntry) -> Result<Self> {
        let (language, resource) = match entry {
            LoadedEntry::Directory(_, directory) => return Self::merge_directory(directory),
            LoadedEntry::File(language, resource) => (language, resource),
        };

        let mut messages = HashMap::new();
        Self::merge_file(language, resource, &mut messages)?;
        Ok(MergedModule {
            sub_modules: Vec::new(),
            messages,
        })
    }

    fn merge_directory(directory: &'a LoadedDirectory) -> Result<Self> {
        let mut sub_modules = Vec::new();
        for (name, sub_directory) in directory.directories() {
            sub_modules.push((name, Self::merge_directory(sub_directory)?));
        }

        let mut messages = HashMap::new();
        for (language, resource) in directory.resources() {
            Self::merge_file(language, resource, &mut messages)?;
        }

        Ok(MergedModule {
            sub_modules,
            messages,
        })
    }

    fn merge_file(
        language: &'a LanguageTag<'static>,
        resource: &'a FluentResource,
        messages: &mut HashMap<&'a FluentIdentifier, MergedMessage<'a>>,
    ) -> Result<()> {
        for entry in resource.entries() {
            let message = match entry {
                FluentEntry::Message(message) => message,
                _ => continue,
            };

            if message.attributes().len() > 0 {
                return Err(Error::new("attributes are not currently supported"));
            }

            let pattern = message.pattern().unwrap();
            let name = message.name();

            let message = match messages.get_mut(name) {
                Some(message) => message,
                None => {
                    messages.insert(name, MergedMessage::new());
                    messages.get_mut(name).unwrap()
                }
            };

            message.insert(name, language, pattern, resource)?;
        }

        Ok(())
    }
}
