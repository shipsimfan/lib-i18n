use crate::{FluentComment, FluentPosition};
use alloc::string::String;

impl FluentComment {
    /// Creates a new [`FluentComment`]
    pub fn new<P: Into<FluentPosition>, S: Into<String>>(
        position: P,
        hashes: u8,
        content: S,
    ) -> Self {
        assert!(hashes > 0);
        assert!(hashes <= 3);

        let position = position.into();
        let content = content.into();

        FluentComment {
            position,
            hashes,
            content,
        }
    }
}
