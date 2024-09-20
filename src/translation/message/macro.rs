/// Creates a [`Message`](`crate::translation::Message`)
#[macro_export]
macro_rules! message {
    ($arguments: ty, |$args: ident, $f: ident| $message: expr) => {{
        fn __inner($args: &$arguments, $f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            $message
        }

        $crate::translation::Message::new(__inner)
    }};
    ($arguments: ty, $message: literal) => {{
        fn __inner(_: &$arguments, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::std::fmt::Display::fmt(&$message, f)
        }

        $crate::translation::Message::new(__inner)
    }};
    ($arguments: ty, $message: path) => {
        $crate::translation::Message::<$arguments>::new($message)
    };
}
