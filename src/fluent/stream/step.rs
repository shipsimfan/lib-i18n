use crate::fluent::{Parse, Stream};

impl<'a> Stream<'a> {
    /// Runs `f` with a peeking stream, advancing this stream only if [`Some`] is returned
    pub fn step<T, F: FnOnce(&mut Stream<'a>) -> Option<T>>(&mut self, f: F) -> Option<T> {
        let mut stream = self.clone();
        if let Some(value) = f(&mut stream) {
            *self = stream;
            return Some(value);
        }

        None
    }

    /// Attempts to parse `T` from the stream, only advancing if the parse succeeds
    pub fn step_parse<T: Parse>(&mut self) -> Option<T> {
        self.step(Parse::parse)
    }
}
