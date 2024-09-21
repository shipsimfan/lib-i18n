use crate::locale::InvalidLanguageTag;

/// An error that can occur while trying to get the current language
#[derive(Debug, PartialEq, Eq)]
pub enum CurrentLanguageError<T: 'static + std::error::Error> {
    /// The error is from the os
    OS(T),

    /// The language tag provided was invalid
    InvalidLanguageTag(InvalidLanguageTag),
}

impl<T: std::error::Error> std::error::Error for CurrentLanguageError<T> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            CurrentLanguageError::OS(error) => Some(error),
            CurrentLanguageError::InvalidLanguageTag(error) => Some(error),
        }
    }
}

impl<T: std::error::Error> std::fmt::Display for CurrentLanguageError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrentLanguageError::OS(error) => std::fmt::Display::fmt(error, f),
            CurrentLanguageError::InvalidLanguageTag(error) => error.fmt(f),
        }
    }
}

impl<T: std::error::Error> From<InvalidLanguageTag> for CurrentLanguageError<T> {
    fn from(value: InvalidLanguageTag) -> Self {
        CurrentLanguageError::InvalidLanguageTag(value)
    }
}
