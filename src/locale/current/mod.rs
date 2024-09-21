mod language;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

pub use language::CURRENT_LANGUAGE;

#[cfg(all(target_os = "linux", feature = "std"))]
pub use linux::{get_current_language, CurrentLanguageError};

#[cfg(all(target_os = "windows", feature = "alloc"))]
pub use windows::{get_current_language, CurrentLanguageError};
