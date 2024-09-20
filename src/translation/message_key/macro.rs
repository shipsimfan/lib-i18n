/// Creates a [`MessageKey`](crate::translation::MessageKey)
#[macro_export]
macro_rules! message_key {
    (
        $(#[$meta: meta])*
        $name: ident { $($arguments: tt)* } [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        #[doc = ::std::concat!("Arguments for [`", ::std::stringify!($name), "`]")]
        struct $name {
            $arguments
        }

        $(#[$meta])*
        const $name: $crate::translation::MessageKey<$name> = $crate::translation::MessageKey::new(&[$(
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
        const $name: $crate::translation::MessageKey<$arguments> = $crate::translation::MessageKey::new(&[$(
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
        const $name: $crate::translation::MessageKey<()> = $crate::translation::MessageKey::new(&[$(
            ($tag, $crate::message!((), $($message)*)),
        )*]);
    };
    (
        $(#[$meta: meta])*
        $vis: vis $name: ident { $($arguments: tt)* } [
            $($tag: path => { $($message: tt)* },)*
        ]
    ) => {
        #[doc = ::std::concat!("Arguments for [`", ::std::stringify!($name), "`]")]
        $vis struct $name {
            $arguments
        }

        $(#[$meta])*
        $vis const $name: $crate::translation::MessageKey<$name> = $crate::translation::MessageKey::new(&[$(
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
        $vis const $name: $crate::translation::MessageKey<$arguments> = $crate::translation::MessageKey::new(&[$(
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
        $vis const $name: $crate::translation::MessageKey<()> = $crate::translation::MessageKey::new(&[$(
            ($tag, $crate::message!((), $($message)*)),
        )*]);
    };
}
