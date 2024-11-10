/// Creates a [`MessageKey`](crate::translation::MessageKey)
#[macro_export]
macro_rules! message_key {
    (
        $(#[$meta: meta])*
        $name: ident { $($arguments: tt)* } [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        #[doc = ::core::concat!("Arguments for [`", ::core::stringify!($name), "`]")]
        #[allow(non_camel_case_types)]
        struct $name {
            $($arguments)*
        }

        $(#[$meta])*
        static $name: $crate::MessageKey<$name> = $crate::MessageKey::new(&[$(
            ($tag, $crate::message!($name, $($message)*)),
        )*]);
    };

    (
        $(#[$meta: meta])*
        $name: ident ($($generics: tt)*) { $($arguments: tt)* } [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        #[doc = ::core::concat!("Arguments for [`", ::core::stringify!($name), "`]")]
        #[allow(non_camel_case_types)]
        struct $name<$($generics)*> {
            $($arguments)*
        }

        $(#[$meta])*
        static $name: $crate::MessageKey<$name> = $crate::MessageKey::new(&[$(
            ($tag, $crate::message!($name, $($message)*)),
        )*]);
    };

    (
        $(#[$meta: meta])*
        $name: ident $arguments: path [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        $(#[$meta])*
        static $name: $crate::MessageKey<$arguments> = $crate::MessageKey::new(&[$(
            ($tag, $crate::message!($arguments, $($message)*)),
        )*]);
    };

    (
        $(#[$meta: meta])*
        $name: ident [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        $(#[$meta])*
        static $name: $crate::MessageKey<()> = $crate::MessageKey::new(&[$(
            ($tag, $crate::message!((), $($message)*)),
        )*]);
    };

    (
        $(#[$meta: meta])*
        $vis: vis $name: ident { $($arguments: tt)* } [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        #[doc = ::core::concat!("Arguments for [`", ::core::stringify!($name), "`]")]
        #[allow(non_camel_case_types)]
        $vis struct $name {
            $($arguments)*
        }

        $(#[$meta])*
        $vis static $name: $crate::MessageKey<$name> = $crate::MessageKey::new(&[$(
            ($tag, $crate::message!($name, $($message)*)),
        )*]);
    };

    (
        $(#[$meta: meta])*
        $vis: vis $name: ident ($($generics: tt)*) { $($arguments: tt)* } [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        #[doc = ::core::concat!("Arguments for [`", ::core::stringify!($name), "`]")]
        #[allow(non_camel_case_types)]
        $vis struct $name<$($generics)*> {
            $($arguments)*
        }

        $(#[$meta])*
        $vis static $name: $crate::MessageKey<$name> = $crate::MessageKey::new(&[$(
            ($tag, $crate::message!($name, $($message)*)),
        )*]);
    };

    (
        $(#[$meta: meta])*
        $vis: vis $name: ident $arguments: path [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        $(#[$meta])*
        $vis static $name: $crate::MessageKey<$arguments> = $crate::MessageKey::new(&[$(
            ($tag, $crate::message!($arguments, $($message)*)),
        )*]);
    };

    (
        $(#[$meta: meta])*
        $vis: vis $name: ident [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        $(#[$meta])*
        $vis static $name: $crate::MessageKey<()> = $crate::MessageKey::new(&[$(
            ($tag, $crate::message!((), $($message)*)),
        )*]);
    };
}
