use crate::fluent::FluentComment;

impl core::fmt::Display for FluentComment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for _ in 0..self.hashes {
            f.write_str("#")?;
        }

        writeln!(f, " {}", self.content)
    }
}
