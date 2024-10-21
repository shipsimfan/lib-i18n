//! Utilities for parsing Fluent files

mod parse;
mod position;
mod stream;

mod blank;
mod entry;
mod line_end;
mod resource;

use parse::Parse;
use stream::{Stream, StreamCollector};

pub use parse::parse;
pub use position::FluentPosition;

#[cfg(feature = "std")]
pub use parse::parse_file;

use blank::{BlankBlock, BlankInline};
use line_end::LineEnd;

pub use entry::*;
pub use resource::FluentResource;
