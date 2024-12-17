use crate::{LoadedDirectory, LoadedEntry};
use i18n_locale::LanguageTag;
use proc_macro_util::{Error, Result};
use std::path::Path;

impl LoadedEntry {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Option<Self>> {
        let path = path.as_ref();

        let name = match path.file_stem() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                return Err(Error::new(format_args!(
                    "invalid path \"{}\", missing name",
                    path.display()
                )))
            }
        };

        if path.is_file() {
            match match path.extension() {
                Some(extension) => extension,
                None => return Ok(None),
            }
            .to_string_lossy()
            .as_ref()
            {
                "ftl" => {}
                _ => return Ok(None),
            };

            let name = LanguageTag::new(name.as_bytes()).map_err(|error| {
                Error::new(format_args!(
                    "invalid language tag \"{}\" for \"{}\" - {}",
                    name,
                    path.display(),
                    error
                ))
            })?;

            i18n_fluent::parse_file(path)
                .map(|resource| Some(LoadedEntry::File(name, resource)))
                .map_err(|error| {
                    Error::new(format_args!(
                        "unable to read \"{}\" - {}",
                        path.display(),
                        error
                    ))
                })
        } else {
            LoadedDirectory::load(path)
                .map(|directory| Some(LoadedEntry::Directory(name, directory)))
        }
    }
}
