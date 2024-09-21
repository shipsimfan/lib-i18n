/// Creates a [`Message`](`crate::translation::Message`)
#[macro_export]
macro_rules! message {
    ($arguments: ty, |$args: ident, $f: ident| $message: expr) => {{
        $crate::translation::Message::new(|| $message)
    }};
    ($arguments: ty, $message: literal) => {{
        $crate::translation::Message::new(|| ::core::fmt::Display::fmt(&$message, f))
    }};
    ($arguments: ty, $message: path) => {
        $crate::translation::Message::<$arguments>::new($message)
    };
}
