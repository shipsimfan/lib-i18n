use crate::fluent::{FluentAttribute, FluentIdentifier, FluentPattern, FluentPosition};

impl FluentAttribute {
    /// Creates a new [`FluentAttribute`]
    pub fn new<P: Into<FluentPosition>, S: Into<String>, Pa: Into<FluentPattern>>(
        position: P,
        name: S,
        pattern: Pa,
    ) -> Self {
        let position = position.into();
        let pattern = pattern.into();

        let mut name_position = position;
        name_position.inc('-');

        assert!(name_position < pattern.position());

        FluentAttribute {
            position,
            name: FluentIdentifier::new(name_position, name),
            pattern,
        }
    }
}
