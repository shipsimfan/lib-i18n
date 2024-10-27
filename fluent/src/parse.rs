use crate::{FluentResource, Stream};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

/// An item which can be parsed from a fluent file
pub(crate) trait Parse: 'static + Sized {
    fn parse(stream: &mut Stream) -> Option<Self>;
}

/// Parses `source` into a [`FluentResource`]
pub fn parse<S: AsRef<str>>(source: S) -> FluentResource {
    let source = source.as_ref();
    let mut stream = Stream::new(source);
    FluentResource::parse(&mut stream).unwrap()
}

/// Parses the file at `path` into a [`FluentResource`]
#[cfg(feature = "std")]
pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> Result<FluentResource, std::io::Error> {
    let source = std::fs::read_to_string(path)?;
    Ok(parse(source))
}

impl Parse for char {
    fn parse(stream: &mut Stream) -> Option<Self> {
        stream.next()
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse(stream: &mut Stream) -> Option<Self> {
        stream.parse().map(|element| Box::new(element))
    }
}
