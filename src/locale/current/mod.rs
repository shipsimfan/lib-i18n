mod language;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

pub use language::CURRENT_LANGUAGE;

#[cfg(target_os = "linux")]
use linux as os;

#[cfg(target_os = "windows")]
use windows as os;

pub use os::{get_current_language, Error};
