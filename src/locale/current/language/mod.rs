use crate::locale::{self, LanguageTag};
use std::sync::LazyLock;

mod error;

/// The current language set by the user from the OS
pub static CURRENT_LANGUAGE: LazyLock<Option<LanguageTag>> =
    LazyLock::new(|| locale::get_current_language());
