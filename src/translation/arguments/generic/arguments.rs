use crate::translation::{Argument, Arguments, GenericArguments};

impl Arguments for GenericArguments {
    fn get_argument<'a>(&'a self, _: &str) -> Option<&'a Argument> {
        todo!()
    }
}
