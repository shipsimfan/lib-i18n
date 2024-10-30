use crate::LanguageTag;
use win32::{try_get_last_error, GetUserDefaultLocaleName, LOCALE_NAME_MAX_LENGTH};

/// An error that can occur while trying to get the current language
pub type CurrentLanguageError = crate::current::language::CurrentLanguageError<win32::Error>;

/// Get the current language the user has selected
pub fn get_current_language() -> Result<LanguageTag<'static>, CurrentLanguageError> {
    let mut buffer = [0; LOCALE_NAME_MAX_LENGTH as usize];
    let len = try_get_last_error!(GetUserDefaultLocaleName(
        buffer.as_mut_ptr(),
        buffer.len() as _
    ))
    .map_err(CurrentLanguageError::OS)?;

    let tag = alloc::string::String::from_utf16_lossy(&buffer[..len as usize - 1]);
    LanguageTag::new(tag.as_bytes()).map_err(Into::into)
}
