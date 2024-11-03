use i18n_fluent::{FluentComment, FluentInlineText, FluentJunk, FluentMessage, FluentResource};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentComment::new((1, 1), 2, "OK"));
    target.push(FluentMessage::new_with(
        (3, 1),
        "bracket-inline",
        FluentInlineText::new((3, 18), "[Value]"),
        Vec::new(),
    ));
    target.push(FluentMessage::new_with(
        (4, 1),
        "dot-inline",
        FluentInlineText::new((4, 14), ".Value"),
        Vec::new(),
    ));
    target.push(FluentMessage::new_with(
        (5, 1),
        "star-inline",
        FluentInlineText::new((5, 15), "*Value"),
        Vec::new(),
    ));

    target.push(FluentComment::new((7, 1), 2, "ERRORS"));
    target.push(FluentJunk::new((9, 1), "bracket-newline =\n    [Value]\n"));
    target.push(FluentJunk::new((11, 1), "dot-newline =\n    .Value\n"));
    target.push(FluentJunk::new((13, 1), "star-newline =\n    *Value"));

    target
}
