/// Creates a [`Message`](`crate::Message`)
#[macro_export]
macro_rules! message {
    (
        $(#[$meta: meta])*
        $arguments: ty, $vis: vis $name: ident $(<$lifetime: lifetime>)* |$args: ident, $f: ident| $message: expr
    ) => {
        $(#[$meta])*
        fn $name$(<$lifetime>)*($args: &$arguments, $f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            $message
        }
    };

    (
        $(#[$meta: meta])*
        $arguments: ty, $vis: vis $name: ident $(<$lifetime: lifetime>)* $format: literal $(, $param: ident)*
    ) => {
        $(#[$meta])*
        fn $name$(<$lifetime>)*(args: &$arguments, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::write!(f, $format $(, args.$param)*)
        }
    };

    (
        $(#[$meta: meta])*
        $arguments: ty, $vis: vis $name: ident $(<$lifetime: lifetime>)* = $message: expr
    ) => {
        $(#[$meta])*
        fn $name$(<$lifetime>)*(_: &$arguments, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Display::fmt($message, f)
        }
    };

    ($arguments: ty, $message: path) => {
        $message
    };
}
