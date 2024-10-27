use crate::{FluentPattern, Parse, Stream};

#[cfg(not(feature = "std"))]
use alloc::vec;

impl Parse for FluentPattern {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let mut elements = vec![stream.parse()?];

        while let Some(element) = stream.step_parse() {
            elements.push(element);
        }

        Some(FluentPattern { elements })
    }
}
