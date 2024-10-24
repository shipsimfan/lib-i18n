use i18n::fluent::{
    FluentComment, FluentInlineText, FluentJunk, FluentMessage, FluentPattern, FluentResource,
};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentComment::new((1, 1), 2, "OK"));
    target.push(FluentMessage::new(
        (3, 1),
        "bracket-inline",
        Some(FluentPattern::new(vec![FluentInlineText::new(
            (3, 18),
            "[Value]",
        )
        .into()])),
        Vec::new(),
    ));
    target.push(FluentMessage::new(
        (4, 1),
        "dot-inline",
        Some(FluentPattern::new(vec![FluentInlineText::new(
            (4, 14),
            ".Value",
        )
        .into()])),
        Vec::new(),
    ));
    target.push(FluentMessage::new(
        (5, 1),
        "star-inline",
        Some(FluentPattern::new(vec![FluentInlineText::new(
            (5, 15),
            "*Value",
        )
        .into()])),
        Vec::new(),
    ));

    target.push(FluentComment::new((7, 1), 2, "ERRORS"));
    target.push(FluentJunk::new((9, 1), "bracket-newline =\n    [Value]\n"));
    target.push(FluentJunk::new((11, 1), "dot-newline =\n    .Value\n"));
    target.push(FluentJunk::new((13, 1), "star-newline =\n    *Value"));

    target
}
