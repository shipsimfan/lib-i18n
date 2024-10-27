use crate::FluentResource;
use alloc::vec::Vec;

impl FluentResource {
    /// Creates a new empty [`FluentResource`]
    pub const fn new() -> Self {
        FluentResource {
            entries: Vec::new(),
        }
    }
}
