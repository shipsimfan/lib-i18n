use crate::{FluentIdentifier, FluentPosition, FluentVariableReference};

#[cfg(not(feature = "std"))]
use alloc::string::String;

impl FluentVariableReference {
    /// Creates a new [`FluentVariableReference`]
    pub fn new<P: Into<FluentPosition>, S: Into<String>>(position: P, name: S) -> Self {
        let position = position.into();

        let mut name_position = position;
        name_position.inc('$');

        FluentVariableReference {
            position,
            name: FluentIdentifier::new(name_position, name),
        }
    }
}
