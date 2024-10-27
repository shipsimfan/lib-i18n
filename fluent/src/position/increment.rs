use crate::FluentPosition;

const TAB_SIZE: u32 = 4;

impl FluentPosition {
    /// Increments this position based on `c`
    pub(crate) fn inc(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else if c == '\t' {
            if self.column % TAB_SIZE == 0 {
                self.column += TAB_SIZE;
            } else {
                self.column = self.column.next_multiple_of(TAB_SIZE);
            }
        } else {
            self.column += 1;
        }
    }
}
