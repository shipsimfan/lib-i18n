use crate::fluent::Stream;

impl<'a> Stream<'a> {
    /// Returns the next character in stream without advacing it
    pub(in crate::fluent) fn peek(&self) -> Option<char> {
        self.peek_n(1)
    }

    /// Returns the character `n` characters ahead in stream without advacing it
    pub(in crate::fluent) fn peek_n(&self, n: usize) -> Option<char> {
        let mut stream = self.clone();
        for _ in 0..n - 1 {
            stream.next();
        }
        stream.next()
    }
}
