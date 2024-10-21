use crate::fluent::{FluentComment, Parse, Stream};
use alloc::string::{String, ToString};

impl Parse for FluentComment {
    fn parse(stream: &mut Stream) -> Option<Self> {
        let position = stream.position();

        let mut hashes = 0;
        while let Some(c) = stream.next() {
            match c {
                ' ' => break,
                '#' => hashes += 1,
                '\n' => {
                    if hashes == 0 || hashes > 3 {
                        return None;
                    } else {
                        return Some(FluentComment {
                            position,
                            hashes,
                            content: String::new(),
                        });
                    }
                }
                _ => return None,
            }
        }

        if hashes == 0 || hashes > 3 {
            return None;
        }

        let mut collector = stream.begin_collecting();
        while let Some(c) = collector.peek() {
            if c == '\n' {
                break;
            }

            collector.next();
        }

        let content = collector.end().to_string();
        stream.next();

        Some(FluentComment {
            position,
            hashes,
            content,
        })
    }
}
