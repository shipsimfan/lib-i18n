use crate::fluent::{
    FluentAttribute, FluentIdentifier, FluentMessage, FluentPattern, FluentPosition,
};

impl FluentMessage {
    /// Creates a new [`FluentMessage`] without a pattern
    pub fn new<P: Into<FluentPosition>, S: Into<String>>(
        position: P,
        name: S,
        attributes: Vec<FluentAttribute>,
    ) -> Self {
        FluentMessage::new_inner(position.into(), name.into(), None, attributes)
    }

    /// Creates a new [`FluentMessage`] with a pattern
    pub fn new_with<P: Into<FluentPosition>, Pa: Into<FluentPattern>, S: Into<String>>(
        position: P,
        name: S,
        pattern: Pa,
        attributes: Vec<FluentAttribute>,
    ) -> Self {
        FluentMessage::new_inner(
            position.into(),
            name.into(),
            Some(pattern.into()),
            attributes,
        )
    }

    /// Creates a new [`FluentMessage`] directly
    fn new_inner(
        position: FluentPosition,
        name: String,
        pattern: Option<FluentPattern>,
        attributes: Vec<FluentAttribute>,
    ) -> Self {
        assert!(pattern.is_some() || attributes.len() > 0);

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
