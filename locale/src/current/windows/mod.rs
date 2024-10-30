#[cfg(feature = "alloc")]
mod language;

#[cfg(feature = "alloc")]
pub use language::{get_current_language, CurrentLanguageError};
