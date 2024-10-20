use crate::fluent::Stream;

impl<'a> Stream<'a> {
    /// Gets the next character from stream, advancing it
    pub(in crate::fluent) fn next(&mut self) -> Option<char> {
        let c = self.characters.next()?;

        self.index += c.len_utf8();
        self.position.inc(c);

        return Some(c);
    }
}
