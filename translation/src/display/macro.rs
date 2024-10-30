/// Gets a displayable version of the provided key, providing arguments and using the specified
/// locale or getting the user's current locale
#[macro_export]
#[cfg(not(feature = "current"))]
macro_rules! m {
    ($key: path, locale = $locale: expr, $($name: ident => $value: expr),+) => {
        $key.get($locale).display(&$key {$(
            $name: $value,
        )+})
    };

    ($key: expr, locale = $locale: expr) => {
        $key.get($locale).display(&())
    };
}

/// Gets a displayable version of the provided key, providing arguments and using the specified
/// locale or getting the user's current locale
#[macro_export]
#[cfg(feature = "current")]
macro_rules! m {
    ($key: path, locale = $locale: expr, $($name: ident => $value: expr),+) => {
        $key.get($locale).display(&$key {$(
            $name: $value,
        )+})
    };

    ($key: expr, locale = $locale: expr) => {
        $key.get($locale).display(&())
    };
    ($key: expr, $($name: ident => $value: expr),+) => {
        match &*$crate::locale::CURRENT_LANGUAGE {
            Some(language) => $key.get(language.clone()),
            None => $key.default(),
        }.display(&$key {$(
            $name: $value,
        )+})
    };
    ($key: expr) => {
        match &*$crate::locale::CURRENT_LANGUAGE {
            Some(language) => $key.get(language.clone()),
            None => $key.default(),
        }.display(&())
    };
}
