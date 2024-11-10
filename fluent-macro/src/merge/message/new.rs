use crate::MergedMessage;
use std::collections::HashMap;

impl<'a> MergedMessage<'a> {
    /// Creates a new [`MergedMessage`]
    pub fn new() -> Self {
        MergedMessage {
            languages: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}
