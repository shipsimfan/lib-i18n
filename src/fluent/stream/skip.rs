use crate::fluent::Stream;

impl<'a> Stream<'a> {
    /// Skips `n` characters ahead in the stream
    pub fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }
}
