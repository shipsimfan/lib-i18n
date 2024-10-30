/// Creates a [`Message`](`crate::translation::Message`)
#[macro_export]
macro_rules! message {
    ($arguments: ty, |$args: ident, $f: ident| $message: expr) => {{
        $crate::translation::Message::new(|$args, $f| $message)
    }};
    ($arguments: ty, $message: expr) => {{
        $crate::translation::Message::new(|_, f| ::core::fmt::Display::fmt(&$message, f))
    }};
    ($arguments: ty, $message: path) => {
        $crate::translation::Message::<$arguments>::new($message)
    };
}
