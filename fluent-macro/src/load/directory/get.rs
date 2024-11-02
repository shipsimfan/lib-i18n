use crate::LoadedDirectory;
use fluent::FluentResource;
use locale::LanguageTag;

impl LoadedDirectory {
    /// Gets the resources loaded in this directory
    pub fn resources(&self) -> impl Iterator<Item = (&LanguageTag<'static>, &FluentResource)> {
        self.resources
            .iter()
            .map(|(name, resource)| (name, resource))
    }

    /// Gets the sub-directories loaded in this directoy
    pub fn directories(&self) -> impl Iterator<Item = (&str, &LoadedDirectory)> {
        self.directories
            .iter()
            .map(|(name, directory)| (name.as_str(), directory))
    }
}
