use i18n_fluent::{
    FluentComment, FluentInlinePlaceable, FluentJunk, FluentMessage, FluentResource,
    FluentVariableReference,
};

pub fn create_target() -> FluentResource {
    let mut target = FluentResource::new();
    target.push(FluentMessage::new_with(
        (1, 1),
        "key01",
        FluentInlinePlaceable::new((1, 9), FluentVariableReference::new((1, 10), "var")),
        Vec::new(),
    ));
    target.push(FluentMessage::new_with(
        (2, 1),
        "key02",
        FluentInlinePlaceable::new((2, 9), FluentVariableReference::new((2, 13), "var")),
        Vec::new(),
    ));
    target.push(FluentMessage::new_with(
        (3, 1),
        "key03",
        FluentInlinePlaceable::new((3, 9), FluentVariableReference::new((4, 5), "var")),
        Vec::new(),
    ));
    target.push(FluentMessage::new_with(
        (6, 1),
        "key04",
        FluentInlinePlaceable::new((6, 9), FluentVariableReference::new((7, 1), "var")),
        Vec::new(),
    ));

    target.push(FluentComment::new((10, 1), 2, "Errors"));
    target.push(FluentComment::new(
        (12, 1),
        1,
        "ERROR Missing variable identifier",
    ));
    target.push(FluentJunk::new((13, 1), "err01 = {$}\n"));
    target.push(FluentComment::new((14, 1), 1, "ERROR Double $$"));
    target.push(FluentJunk::new((15, 1), "err02 = {$$var}\n"));
    target.push(FluentComment::new(
        (16, 1),
        1,
        "ERROR Invalid first char of the identifier",
    ));
    target.push(FluentJunk::new((17, 1), "err03 = {$-var}"));

    target
}
