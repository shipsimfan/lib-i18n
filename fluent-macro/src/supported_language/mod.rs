use i18n_locale::LanguageTag;
use proc_macro_util::tokens::Identifier;

mod borrow;
mod eq;
mod get;
mod hash;
mod new;
mod to_tokens;

/// A supported language
pub struct SupportedLanguage {
    /// The rendered identifier form of the language
    identifier: Identifier,

    /// The parsed language
    tag: LanguageTag<'static>,
}
