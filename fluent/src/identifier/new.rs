use crate::{FluentIdentifier, FluentPosition};

#[cfg(not(feature = "std"))]
use alloc::string::String;

impl FluentIdentifier {
    /// Creates a new [`FluentIdentifier`] validating the contents
    pub fn new<P: Into<FluentPosition>, S: Into<String>>(position: P, content: S) -> Self {
        let position = position.into();
        let content = content.into();

        let mut chars = content.chars();

        let first = chars.next().unwrap();
        assert!(first.is_ascii_alphabetic());

        for c in chars {
            assert!(c.is_ascii_alphanumeric() || c == '_' || c == '-');
        }

        unsafe { FluentIdentifier::new_unchecked(position, content) }
    }

    /// Creates a new [`FluentIdentifier`] without checking the contents
    ///
    /// # SAFETY
    /// The content must start with one ASCII alphabetic character and the remaining characters
    /// must only contain ASCII alphanumeric characters.
    pub const unsafe fn new_unchecked(position: FluentPosition, content: String) -> Self {
        FluentIdentifier { position, content }
    }
}
