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
        struct $name {
            $arguments
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
        $vis struct $name {
            $arguments
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
