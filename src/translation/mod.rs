//! Utilities for displaying messages in different languages

mod arguments;
mod display;
mod message;
mod message_key;

pub use arguments::{Argument, Arguments, GenericArguments};
pub use display::MessageDisplay;
pub use message::Message;
pub use message_key::{GenericMessageKey, MessageKey};
