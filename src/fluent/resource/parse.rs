use crate::fluent::{FluentResource, Parse, Stream};
use alloc::vec::Vec;

impl Parse for FluentResource {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let mut entries = Vec::new();

        while !stream.empty() {
            if let Some(entry) = stream.parse() {
                entries.push(entry);
            }
        }

        Some(FluentResource { entries })
    }
}
