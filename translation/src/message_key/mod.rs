use messages::Messages;

mod messages;

mod get;
mod r#macro;
mod new;

/// A set of [`Message`](crate::translation::Message)s with the same meaning in multiple languages
pub struct MessageKey<'a, A> {
    /// The set of messages with their corresponding language
    messages: Messages<'a, A>,
}
