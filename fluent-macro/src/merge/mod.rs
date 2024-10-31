use fluent::FluentIdentifier;
use std::collections::HashMap;

mod get;
mod message;

pub use message::MergedMessage;

/// A module with collect messages
pub struct MergedModule<'a> {
    /// The sub-modules of this module
    sub_modules: Vec<MergedModule<'a>>,

    /// The messages in this module
    messages: HashMap<&'a FluentIdentifier, MergedMessage<'a>>,
}
