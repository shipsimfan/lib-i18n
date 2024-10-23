use crate::fluent::{
    FluentAttribute, FluentIdentifier, FluentMessage, FluentPattern, FluentPosition,
};

impl FluentMessage {
    /// Creates a new [`FluentMessage`]
    pub fn new<P: Into<FluentPosition>, S: Into<String>>(
        position: P,
        name: S,
        pattern: Option<FluentPattern>,
        attributes: Vec<FluentAttribute>,
    ) -> Self {
        assert!(pattern.is_some() || attributes.len() > 0);

        let position = position.into();

        if let Some(pattern) = pattern.as_ref() {
            assert!(position < pattern.position());

            if attributes.len() > 0 {
                assert!(pattern.position() < attributes[0].position())
            }
        } else if attributes.len() > 0 {
            assert!(position < attributes[0].position());
        }

        FluentMessage {
            name: FluentIdentifier::new(position, name),
            pattern,
            attributes,
        }
    }
}
