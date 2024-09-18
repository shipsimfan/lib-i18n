use crate::translation::Arguments;
use std::marker::PhantomData;

mod message_key;

/// A [`MessageKey`](crate::translation::MessageKey) that can be built at runtime
pub struct GenericMessageKey<A: Arguments> {
    _arguments: PhantomData<A>,
}
