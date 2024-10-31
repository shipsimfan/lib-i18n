use fluent::FluentResource;
use locale::LanguageTag;

mod get;
mod load;

/// A directory loaded from the file system
pub struct LoadedDirectory {
    /// The loaded resources with their languages
    resources: Vec<(LanguageTag<'static>, FluentResource)>,

    /// The loaded sub-directories
    directories: Vec<(String, LoadedDirectory)>,
}
