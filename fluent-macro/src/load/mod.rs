use fluent::FluentResource;

mod directory;

mod load;

pub use directory::LoadedDirectory;
use locale::LanguageTag;

/// An entry in the filesystem tree that has been loaded
pub enum LoadedEntry {
    /// The entry is a directory
    Directory((String, LoadedDirectory)),

    /// The entry is a file
    File((LanguageTag<'static>, FluentResource)),
}
