use crate::LanguageTag;
use error::NoLanguageFound;

mod error;

pub use error::CurrentLanguageError;

/// The possible sources for the current language, rated from highest to lowest priority
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TagSource {
    #[allow(non_camel_case_types)]
    LC_ALL,
    #[allow(non_camel_case_types)]
    LC_MESSAGES,
    LANGUAGE,
    LANG,
}

/// Get the current language the user has selected
pub fn get_current_language() -> Result<LanguageTag<'static>, CurrentLanguageError> {
    let mut tag = None;
    for (key, value) in std::env::vars_os() {
        let source = match key.as_encoded_bytes() {
            b"LC_ALL" => {
                tag = Some((TagSource::LC_ALL, value));
                break;
            }
            b"LC_MESSAGES" => TagSource::LC_MESSAGES,
            b"LANGUAGE" => TagSource::LANGUAGE,
            b"LANG" => TagSource::LANG,
            _ => continue,
        };

        match &tag {
            Some((old_source, _)) => {
                if *old_source > source {
                    tag = Some((source, value))
                }
            }
            None => tag = Some((source, value)),
        }
    }

    let tag = match tag {
        Some((_, tag)) => tag,
        None => return Err(CurrentLanguageError::OS(NoLanguageFound)),
    };

    // Find first '.' or ':'
    let mut end = tag.len();
    for (i, byte) in tag.as_encoded_bytes().iter().enumerate() {
        if *byte == b'.' || *byte == b':' {
            end = i;
            break;
        }
    }

    LanguageTag::new(&tag.as_encoded_bytes()[..end]).map_err(Into::into)
}
