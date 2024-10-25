use i18n::fluent::{
    FluentComment, FluentInlineText, FluentJunk, FluentMessage, FluentPattern, FluentResource,
    FluentTerm,
};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentComment::new((1, 1), 1, "Standalone Comment"));
    target.push(FluentComment::new((3, 1), 1, "Message Comment"));
    target.push(FluentMessage::new(
        (4, 1),
        "foo",
        Some(FluentPattern::new(vec![FluentInlineText::new(
            (4, 7),
            "Foo",
        )
        .into()])),
        Vec::new(),
    ));
    target.push(FluentComment::new((6, 1), 1, "Term Comment"));
    target.push(FluentComment::new((7, 1), 1, "with a blank last line."));
    target.push(FluentComment::new((8, 1), 1, ""));
    target.push(FluentTerm::new(
        (9, 1),
        "term",
        FluentPattern::new(vec![FluentInlineText::new((9, 9), "Term").into()]),
        Vec::new(),
    ));
    target.push(FluentComment::new((11, 1), 1, "Another standalone"));
    target.push(FluentComment::new((12, 1), 1, ""));
    target.push(FluentComment::new((13, 1), 1, "     with indent"));
    target.push(FluentComment::new((14, 1), 2, "Group Comment"));
    target.push(FluentComment::new((15, 1), 3, "Resource Comment"));
    target.push(FluentComment::new((17, 1), 1, "Errors"));
    target.push(FluentJunk::new((18, 1), "#error\n"));
    target.push(FluentJunk::new((19, 1), "##error\n"));
    target.push(FluentJunk::new((20, 1), "###error"));
    target
}
