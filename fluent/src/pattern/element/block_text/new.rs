use crate::{FluentBlockText, FluentPosition};

#[cfg(not(feature = "std"))]
use alloc::string::String;

impl FluentBlockText {
    /// Creates a new [`FluentBlockText`] validating the contents
    pub fn new<P: Into<FluentPosition>, S: Into<String>>(position: P, content: S) -> Self {
        let content = content.into();

        let c = content.chars().next().unwrap();
        assert!(c != '[' && c != '*' && c != '.');

        for c in content.chars() {
            assert!(c != '{' && c != '}' && c != '\n');
        }

        unsafe { FluentBlockText::new_unchecked(position.into(), content) }
    }

    /// Creates a new [`FluentBlockText`] without checking the contents
    ///
    /// # SAFETY
    /// `content` must be a valid inline-text element starting with any character except '[', '*', or '.'
    pub const unsafe fn new_unchecked(position: FluentPosition, content: String) -> Self {
        FluentBlockText { position, content }
    }
}
