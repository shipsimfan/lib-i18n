use crate::translation::Arguments;
use std::marker::PhantomData;

/// A message translated into a specific language
pub struct Message<A: Arguments> {
    _arguments: PhantomData<A>,
}
