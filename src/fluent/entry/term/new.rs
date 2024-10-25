use crate::fluent::{FluentAttribute, FluentIdentifier, FluentPattern, FluentPosition, FluentTerm};

impl FluentTerm {
    /// Creates a new [`FluentTerm`]
    pub fn new<P: Into<FluentPosition>, S: Into<String>>(
        position: P,
        name: S,
        pattern: FluentPattern,
        attributes: Vec<FluentAttribute>,
    ) -> Self {
        let position = position.into();
        let mut name_position = position;
        name_position.inc('-');

        assert!(name_position < pattern.position());

        if attributes.len() > 0 {
            assert!(pattern.position() < attributes[0].position())
        }

        FluentTerm {
            position,
            name: FluentIdentifier::new(name_position, name),
            pattern,
            attributes,
        }
    }
}
