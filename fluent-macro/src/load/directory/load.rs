use crate::{load::LoadedEntry, LoadedDirectory};
use proc_macro_util::{Error, Result};
use std::path::Path;

impl LoadedDirectory {
    /// Loads the directory at `path`
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let mut resources = Vec::new();
        let mut directories = Vec::new();
        for entry in std::fs::read_dir(path).map_err(|error| {
            Error::new(format_args!(
                "unable to read \"{}\" - {}",
                path.display(),
                error
            ))
        })? {
            let entry = entry.map_err(|error| {
                Error::new(format_args!(
                    "unable to read \"{}\" - {}",
                    path.display(),
                    error
                ))
            })?;

            let path = entry.path();

            match LoadedEntry::load(path)? {
                Some(LoadedEntry::File(name, resource)) => resources.push((name, resource)),
                Some(LoadedEntry::Directory(name, directory)) => {
                    directories.push((name, directory))
                }
                None => {}
            }
        }

        Ok(LoadedDirectory {
            resources,
            directories,
        })
    }
}
