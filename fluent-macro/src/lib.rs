//! Definition of the fluent file loading macro

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod input;
mod load;

mod parse;
mod to_tokens;

use input::{IncludeFluentInput, IncludeFluentOptions};
use load::{LoadedDirectory, LoadedEntry};

proc_macro_util::proc_macro_function!(
    /// Loads Fluent translation files from the specified `path` and creates
    /// [`i18n::translation::MessageKeys`] from each [`fluent::FluentMessage`] found.
    ///
    /// # Format
    /// ```rs
    /// include_fluent!("path" [ , option = value ]*);
    /// ```
    ///
    /// where
    ///  - `path` is a string literal pointing to either a file or directory containing Fluent
    ///    resources
    ///  - `option = value` is any number of option/value pairs separated by commas adjusting the
    ///    behaviour of the output. The options are described below. The `option` is an identifier.
    ///    The `value` is based on the option and may not be required.
    ///
    /// # Description
    /// If `path` points to a single file, the macro loads only that file, making a single language
    /// available based on the name of the file.
    ///
    /// If `path` points to a directory, the macro searches all subdirectories for files with the
    /// ".ftl" extension. Each subdirectory will correspond to a new public submodule, named after
    /// the subdirectory itself. Inside each submodule, message keys with identical identifiers are
    /// merged across files to support different languages.
    ///
    /// Each file in a directory should represent a different language, identified by the file
    /// name. For example, "EN.ftl" will get the language identifier "EN" while "FR-fr.ftl" will get
    /// the identifier "FR-fr".
    ///
    /// # Options
    /// The following options are defined:
    ///  * `fluent` - This option takes a literal string and specifies which language will be used
    ///               as the fallback language. If not provided, "EN" will be used as the fallback
    ///               language.
    include_fluent -> IncludeFluent
);

struct IncludeFluent {}
