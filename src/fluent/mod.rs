//! Utilities for parsing Fluent files

mod parse;
mod position;
mod stream;

mod entry;
mod resource;

use parse::Parse;
use stream::{Stream, StreamCollector};

pub use parse::parse;
pub use position::FluentPosition;

#[cfg(feature = "std")]
pub use parse::parse_file;

pub use entry::*;
pub use resource::FluentResource;
