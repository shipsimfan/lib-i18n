use crate::fluent::{FluentResource, Parse, Stream};

impl Parse for FluentResource {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let mut junk = Vec::new();

        while !stream.empty() {
            junk.push(stream.parse().unwrap());
        }

        Some(FluentResource { junk })
    }
}
