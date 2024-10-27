use fluent::{FluentComment, FluentJunk, FluentResource};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentComment::new((1, 1), 2, "Two adjacent Junks."));
    target.push(FluentJunk::new((2, 1), "err01 = {1x}\n"));
    target.push(FluentJunk::new((3, 1), "err02 = {2x}\n\n"));
    target.push(FluentComment::new((5, 1), 1, "A single Junk."));
    target.push(FluentJunk::new((6, 1), "err03 = {1x\n2\n\n"));
    target.push(FluentComment::new((9, 1), 1, "A single Junk."));
    target.push(FluentJunk::new(
        (10, 1),
        "ą=Invalid identifier\nć=Another one\n\n",
    ));
    target.push(FluentComment::new(
        (13, 1),
        1,
        "The COMMENT ends this junk.",
    ));
    target.push(FluentJunk::new((14, 1), "err04 = {\n"));
    target.push(FluentComment::new((15, 1), 1, "COMMENT"));
    target.push(FluentComment::new(
        (17, 1),
        1,
        "The COMMENT ends this junk.",
    ));
    target.push(FluentComment::new(
        (18, 1),
        1,
        "The closing brace is a separate Junk.",
    ));
    target.push(FluentJunk::new((19, 1), "err04 = {\n"));
    target.push(FluentComment::new((20, 1), 1, "COMMENT"));
    target.push(FluentJunk::new((21, 1), "}"));
    target
}
