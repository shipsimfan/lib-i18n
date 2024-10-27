use crate::{FluentInlineText, FluentPosition};

#[cfg(not(feature = "std"))]
use alloc::string::String;

impl FluentInlineText {
    /// Creates a new [`FluentInlineText`] validating the contents
    pub fn new<P: Into<FluentPosition>, S: Into<String>>(position: P, content: S) -> Self {
        let position = position.into();
        let content = content.into();

        assert!(content.len() > 0);
        for c in content.chars() {
            assert!(c != '{' && c != '}' && c != '\n');
        }

        unsafe { FluentInlineText::new_unchecked(position, content) }
    }

    /// Creates a new [`FluentInlineText`] without checking the contents
    ///
    /// # SAFETY
    /// The content must not contain any newlines and opening or closing curly braces ('{' or '}')
    pub const unsafe fn new_unchecked(position: FluentPosition, content: String) -> Self {
        FluentInlineText { position, content }
    }
}
