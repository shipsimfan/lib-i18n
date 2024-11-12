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
    ($key: ident, locale = $locale: expr, $($name: ident => $value: expr),+) => {
        <$key as $crate::MessageKey>::get($locale, $key {$(
            $name: $value,
        )+})
    };

    ($key: ident, locale = $locale: expr) => {
        <$key as $crate::MessageKey>::get($locale, ())
    };
    ($key: ident, $($name: ident => $value: expr),+) => {{
        match &*$crate::locale::CURRENT_LANGUAGE {
            Some(language) => <$key as $crate::MessageKey>::get(language.clone(), $key { $(
            $name: $value,
        )+ }),
            None => <$key as $crate::MessageKey>::default($key { $(
            $name: $value,
        )+ }),
        }
    }};
    ($key: ident) => {
        match &*$crate::locale::CURRENT_LANGUAGE {
            Some(language) => <$key as $crate::MessageKey>::get(language.clone(), ()),
            None => <$key as $crate::MessageKey>::default(()),
        }
    };
}
