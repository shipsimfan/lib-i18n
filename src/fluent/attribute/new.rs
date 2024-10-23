use crate::fluent::{FluentAttribute, FluentIdentifier, FluentPattern, FluentPosition};

impl FluentAttribute {
    /// Creates a new [`FluentAttribute`]
    pub fn new<P: Into<FluentPosition>, S: Into<String>, Pa: Into<FluentPattern>>(
        position: P,
        name: S,
        pattern: Pa,
    ) -> Self {
        FluentAttribute {
            name: FluentIdentifier::new(position, name),
            pattern: pattern.into(),
        }
    }
}
