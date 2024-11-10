/// An error that can occur while trying to get the current language
pub type CurrentLanguageError = crate::current::language::CurrentLanguageError<NoLanguageFound>;

/// No language was found
#[derive(Debug)]
pub struct NoLanguageFound;

impl std::error::Error for NoLanguageFound {}

impl std::fmt::Display for NoLanguageFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("no language found")
    }
}
