//! Definition of the fluent file loading macro

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod parse;
mod to_tokens;

proc_macro_util::proc_macro_function!(
    /// Loads Fluent translation files from the specified `path` and creates
    /// [`i18n::translation::MessageKeys`] from each [`fluent::FluentMessage`] found.
    ///
    /// # Format
    /// ```rs
    /// include_fluent!("path");
    /// ```
    ///
    /// where `path` is a string literal pointing to either a file or directory containing Fluent
    /// resources.
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
    /// TODO: Support setting fallback language
    include_fluent -> IncludeFluent
);

struct IncludeFluent {}
