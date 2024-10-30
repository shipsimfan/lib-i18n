#[cfg(feature = "std")]
mod language;

#[cfg(feature = "std")]
pub use language::{get_current_language, CurrentLanguageError};
