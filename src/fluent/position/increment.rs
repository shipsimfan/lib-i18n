use crate::fluent::FluentPosition;

impl FluentPosition {
    /// Increments this position based on `c`
    pub(in crate::fluent) fn inc(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else if !c.is_control() {
            self.column += 1;
        }
    }
}
