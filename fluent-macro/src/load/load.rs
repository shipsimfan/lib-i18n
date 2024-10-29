use crate::{LoadedDirectory, LoadedEntry};
use proc_macro_util::{Error, Result};
use std::path::Path;

impl LoadedEntry {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Option<Self>> {
        let path = path.as_ref();

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => return Err(Error::new("invalid path")),
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

            fluent::parse_file(path)
                .map(|resource| Some(LoadedEntry::File((name, resource))))
                .map_err(|error| {
                    Error::new(format_args!(
                        "unable to read \"{}\" - {}",
                        path.display(),
                        error
                    ))
                })
        } else {
            LoadedDirectory::load(path)
                .map(|directory| Some(LoadedEntry::Directory((name, directory))))
        }
    }
}
