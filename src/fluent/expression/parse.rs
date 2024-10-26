use crate::fluent::{FluentExpression, Parse, Stream};

impl Parse for FluentExpression {
    fn parse(stream: &mut Stream) -> Option<Self> {
        None
    }
}
