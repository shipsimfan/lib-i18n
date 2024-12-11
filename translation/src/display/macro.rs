/// Gets a displayable version of the provided key, providing arguments and using the specified
/// locale or getting the user's current locale
#[macro_export]
#[cfg(not(feature = "current"))]
macro_rules! m {
    ($key: path, locale = $locale: expr, $($name: ident => $value: expr),+) => {
        use $key as __MESSAGE;
        $key.get($locale).display(&__MESSAGE {$(
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
    ($key: path, locale = $locale: expr, $($name: ident $(=> $value: expr)*),+) => {
        use $key as __MESSAGE;
        <$key as $crate::MessageKey>::get($locale, __MESSAGE {$(
            $name $(: $value)*,
        )+})
    };

    ($key: path, locale = $locale: expr) => {
        <$key as $crate::MessageKey>::get($locale, ())
    };
    ($key: path, $($name: ident $(=> $value: expr)*),+) => {{
        use $key as __MESSAGE;
        match &*$crate::locale::CURRENT_LANGUAGE {
            Some(language) => <$key as $crate::MessageKey>::get(language.clone(), __MESSAGE { $(
            $name $(: $value)*,
        )+ }),
            None => <$key as $crate::MessageKey>::default(__MESSAGE { $(
            $name $(: $value)*,
        )+ }),
        }
    }};
    ($key: path) => {
        match &*$crate::locale::CURRENT_LANGUAGE {
            Some(language) => <$key as $crate::MessageKey>::get(language.clone(), ()),
            None => <$key as $crate::MessageKey>::default(()),
        }
    };
}
