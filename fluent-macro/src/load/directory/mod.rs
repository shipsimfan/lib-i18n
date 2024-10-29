use fluent::FluentResource;

mod load;

/// A directory loaded from the file system
pub struct LoadedDirectory {
    /// The loaded resources with their languages
    resources: Vec<(String, FluentResource)>,

    /// The loaded sub-directories
    directories: Vec<(String, LoadedDirectory)>,
}
